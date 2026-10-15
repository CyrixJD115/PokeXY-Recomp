"""Dataset model + generic JSON validation tests."""

import json
import tempfile
import unittest
from pathlib import Path

from xyheatmap.dataset import (
    FLAG_COVERED,
    FLAG_HINT,
    dataset_from_dict,
    dataset_to_api,
    load_dataset,
    parse_addr,
)
from xyheatmap.errors import HeatmapError

from .util import sample_dataset, sample_dataset_dict


class DatasetFromDictTests(unittest.TestCase):
    def test_minimal_roundtrip(self):
        ds = dataset_from_dict(sample_dataset_dict())
        self.assertEqual(ds.title, "test corpus")
        self.assertEqual([m.name for m in ds.modules], ["DllBag", "DllBattle", "DllTitle"])

    def test_cells_sorted_by_address(self):
        ds = dataset_from_dict(
            {"modules": [{"name": "M", "cells": [{"addr": "0x300"}, {"addr": "0x100"}, {"addr": "0x200"}]}]}
        )
        self.assertEqual([c.addr for c in ds.module("M").cells], [0x100, 0x200, 0x300])

    def test_tier_escalation_and_flags(self):
        ds = dataset_from_dict(sample_dataset_dict())
        bag = ds.module("DllBag")
        c0, c1, c2 = bag.cells
        self.assertEqual(c0.tier, 2)
        self.assertTrue(c0.covered and c0.flags & FLAG_COVERED)
        self.assertEqual(c1.tier, 1)
        self.assertTrue(c1.hint and c1.flags & FLAG_HINT)
        self.assertEqual(c2.tier, 3)
        self.assertEqual(c2.name, "DoOpen")

    def test_hex_and_decimal_addr_strings(self):
        self.assertEqual(parse_addr("0x1A0", "t"), 0x1A0)
        self.assertEqual(parse_addr("384", "t"), 384)
        self.assertEqual(parse_addr(4112, "t"), 4112)

    def test_duplicate_address_rejected(self):
        with self.assertRaises(HeatmapError) as cm:
            dataset_from_dict(
                {"modules": [{"name": "M", "cells": [{"addr": 8}, {"addr": "0x8"}]}]}
            )
        self.assertIn("duplicate address", str(cm.exception))

    def test_bad_tier_rejected(self):
        with self.assertRaises(HeatmapError) as cm:
            dataset_from_dict({"modules": [{"name": "M", "cells": [{"addr": 1, "tier": "gold"}]}]})
        self.assertIn("tier must be one of", str(cm.exception))

    def test_bad_addr_rejected(self):
        for bad in ("xyz", -4, None, True):
            with self.assertRaises(HeatmapError):
                parse_addr(bad, "t")

    def test_empty_modules_rejected(self):
        with self.assertRaises(HeatmapError):
            dataset_from_dict({"modules": []})

    def test_non_object_cell_rejected(self):
        with self.assertRaises(HeatmapError):
            dataset_from_dict({"modules": [{"name": "M", "cells": [7]}]})

    def test_aggregates(self):
        ds = dataset_from_dict(sample_dataset_dict())
        self.assertEqual(ds.total_cells(), 46)
        counts = ds.tier_counts()
        self.assertEqual(sum(counts.values()), 46)
        self.assertEqual(ds.relink_summary(), (1, 2))
        ns = ds.namespace_stats()
        self.assertIn("ui::title", ns)
        self.assertEqual(ns["ui::title"]["decompiled"], 1)

    def test_api_projection_shape(self):
        api = dataset_to_api(dataset_from_dict(sample_dataset_dict()))
        self.assertEqual(api["title"], "test corpus")
        row = api["modules"][0]["cells"][0]
        self.assertEqual(len(row), 6)
        self.assertEqual(row[0], 0x180)


class LoadDatasetTests(unittest.TestCase):
    def test_load_from_file(self):
        with tempfile.TemporaryDirectory() as td:
            path = Path(td) / "ds.json"
            path.write_text(json.dumps(sample_dataset_dict()))
            ds = load_dataset(str(path))
            self.assertEqual(ds.module("DllBag").name, "DllBag")

    def test_invalid_json_message(self):
        with tempfile.TemporaryDirectory() as td:
            path = Path(td) / "bad.json"
            path.write_text("{not json")
            with self.assertRaises(HeatmapError) as cm:
                load_dataset(str(path))
            self.assertIn("not valid JSON", str(cm.exception))

    def test_missing_file_message(self):
        with self.assertRaises(HeatmapError) as cm:
            load_dataset("/definitely/not/here.json")
        self.assertIn("cannot read dataset", str(cm.exception))


class ModuleLookupTests(unittest.TestCase):
    def test_case_insensitive_fallback(self):
        ds = sample_dataset()
        self.assertEqual(ds.module("dllbag").name, "DllBag")

    def test_unknown_module_lists_known(self):
        ds = sample_dataset()
        with self.assertRaises(HeatmapError) as cm:
            ds.module("Nope")
        self.assertIn("DllBattle", str(cm.exception))


if __name__ == "__main__":
    unittest.main()
