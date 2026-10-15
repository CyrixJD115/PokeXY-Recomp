"""Collector tests against a synthetic (tiny) source tree — no .Data needed."""

import tempfile
import unittest
from pathlib import Path

from xyheatmap.collect import FileDatasetSource, ProjectSource, discover_data_root
from xyheatmap.errors import HeatmapError

ENTRIES_C = """#include "recomp.h"
RECOMP_EXPORT const uint32_t recomp_entry_count = 10;
RECOMP_EXPORT const Entry recomp_entries[] = {
    {0x00100000u, f_00100000},
};
static const Module modules[] = {
    {"DllBag", &m001_base, 0x1000u, 2, m001_entries},
};
"""

FUNCTIONS_H = """extern uint32_t recomp_starts[];
void f_00100000(Context *ctx);
void f_00100024(Context *ctx);
"""

M001_H = """extern uint32_t m001_base;
void m001_f_00000180(Context *ctx);
"""

CSV_HEADER = '"Location","Name","Namespace","Mode","Size","Segment"\n'
CODE_CSV = CSV_HEADER + (
    "00100024,FUN_00100024,Global,$a,0000001c,ram\n"
    "00100100,DoThing,game::bag,$a,00000010,ram\n"
)
BAG_CSV = CSV_HEADER + "00000180,FUN_00100180,Global,$a,00000004,ram\n"

STATIC_TSV = "name\toffset\tload_addr\n_ZN9xy_system2bg6System13SetIsCallDrawEb\t0x100785\t0x100785\n"
BAG_TSV = "name\toffset\tload_addr\nBag_Open\t0x1c0\t0x101c0\n"


def build_tree(root: Path) -> None:
    (root / "recomp").mkdir(parents=True)
    (root / "pipeline" / "symbols").mkdir(parents=True)
    (root / "pipeline" / "orig").mkdir(parents=True)
    (root / "pipeline" / "out").mkdir(parents=True)
    (root / "cro_extracted" / "static").mkdir(parents=True)
    (root / "cro_extracted" / "DllBag").mkdir(parents=True)
    (root / "decomp" / "code_c").mkdir(parents=True)
    (root / "decomp" / "cro_c" / "DllBag").mkdir(parents=True)

    (root / "recomp" / "entries.c").write_text(ENTRIES_C)
    (root / "recomp" / "functions.h").write_text(FUNCTIONS_H)
    (root / "recomp" / "m001_functions.h").write_text(M001_H)
    (root / "pipeline" / "symbols" / "code.bin.csv").write_text(CODE_CSV)
    (root / "pipeline" / "symbols" / "DllBag.cro.csv").write_text(BAG_CSV)
    (root / "cro_extracted" / "static" / "symbols.tsv").write_text(STATIC_TSV)
    (root / "cro_extracted" / "DllBag" / "symbols.tsv").write_text(BAG_TSV)
    (root / "decomp" / "code_c" / "00100000_FUN_00100000.c").write_text("void f(void){}\n")
    (root / "decomp" / "code_c" / "00100024_FUN_00100024.c").write_text("void f(void){}\n")
    (root / "decomp" / "cro_c" / "DllBag" / "00100180_FUN_00100180.c").write_text("void f(void){}\n")
    (root / "pipeline" / "orig" / "code.bin").write_bytes(b"ORIG")
    (root / "pipeline" / "out" / "code.bin").write_bytes(b"ORIG")
    (root / "pipeline" / "orig" / "DllBag.cro").write_bytes(b"ONE")
    (root / "pipeline" / "out" / "DllBag.cro").write_bytes(b"TWO")


class CollectTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        root = Path(self._tmp.name) / ".Data" / "Source_Extracted"
        build_tree(root)
        self.root = root
        self.source = ProjectSource(root)

    def tearDown(self):
        self._tmp.cleanup()

    def test_main_cells_merged(self):
        ds = self.source.dataset()
        main = ds.module("Main")
        addrs = {c.addr for c in main.cells}
        self.assertEqual(
            addrs, {0x00100000, 0x00100024, 0x00100100, 0x00100784},
            "expected covered + csv + decomp + Thumb-stripped export addresses",
        )
        by_addr = {c.addr: c for c in main.cells}
        self.assertTrue(by_addr[0x00100024].covered)
        self.assertEqual(by_addr[0x00100024].size, 0x1C)
        self.assertEqual(by_addr[0x00100024].tier, 1)  # decompiled
        self.assertEqual(by_addr[0x00100100].tier, 2)  # named
        self.assertEqual(by_addr[0x00100100].namespace, "game::bag")
        self.assertEqual(by_addr[0x00100784].name, "_ZN9xy_system2bg6System13SetIsCallDrawEb")
        self.assertEqual(by_addr[0x00100784].namespace, "xy_system::bg::System")

    def test_cro_relative_frame(self):
        ds = self.source.dataset()
        bag = ds.module("DllBag")
        addrs = {c.addr for c in bag.cells}
        self.assertEqual(addrs, {0x180, 0x1C0})
        by_addr = {c.addr: c for c in bag.cells}
        self.assertTrue(by_addr[0x180].covered)
        self.assertEqual(by_addr[0x180].tier, 1)
        self.assertEqual(by_addr[0x1C0].name, "Bag_Open")
        self.assertEqual(by_addr[0x1C0].tier, 2)

    def test_relink_flags(self):
        ds = self.source.dataset()
        self.assertTrue(ds.module("Main").relink_ok)
        self.assertFalse(ds.module("DllBag").relink_ok)

    def test_fingerprint_stable_and_sensitive(self):
        fp1 = self.source.fingerprint()
        fp2 = self.source.fingerprint()
        self.assertEqual(fp1, fp2)
        new = self.root / "decomp" / "code_c" / "00100200_FUN_00100200.c"
        new.write_text("void f(void){}\n")
        self.assertNotEqual(fp1, self.source.fingerprint())

    def test_missing_entries_c_rejected(self):
        with self.assertRaises(HeatmapError):
            ProjectSource(self.root / "decomp")

    def test_discover_data_root(self):
        deep = Path(self._tmp.name) / "a" / "b" / "c"
        deep.mkdir(parents=True, exist_ok=True)
        self.assertEqual(discover_data_root(deep), self.root)

    def test_missing_sources_warn_not_crash(self):
        import shutil

        shutil.rmtree(self.root / "pipeline" / "symbols")
        shutil.rmtree(self.root / "decomp" / "cro_c" / "DllBag")
        ds = self.source.dataset()
        bag = ds.module("DllBag")
        self.assertEqual({c.addr for c in bag.cells}, {0x180, 0x1C0})  # covered + export remain
        self.assertTrue(any("no symbol CSV" in w for w in ds.warnings))


class FileDatasetSourceTests(unittest.TestCase):
    def test_describe_and_load(self):
        with tempfile.TemporaryDirectory() as td:
            p = Path(td) / "ds.json"
            p.write_text('{"modules":[{"name":"M","cells":[{"addr":0}]}]}')
            src = FileDatasetSource(p)
            self.assertEqual(src.dataset().module("M").name, "M")
            self.assertIn("ds.json", src.describe())

    def test_missing_file(self):
        with self.assertRaises(HeatmapError):
            FileDatasetSource(Path("/no/such/ds.json"))


if __name__ == "__main__":
    unittest.main()
