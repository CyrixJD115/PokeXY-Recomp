"""Layout engine: squarified treemap math, cell grids, bar views."""

import math
import unittest

from xyheatmap.config import Config
from xyheatmap.errors import HeatmapError
from xyheatmap.layout import (
    _fit,
    _squarify,
    corpus_grid_scene,
    grid_scene,
    namespaces_scene,
    overview_scene,
    treemap_scene,
)
from xyheatmap.palette import TIER_COLORS
from xyheatmap.scene import Cells, Rect

from .util import sample_dataset


def _rects(scene):
    return [i for i in scene.items if isinstance(i, Rect)]


def _cells(scene):
    return [i for i in scene.items if isinstance(i, Cells)]


class SquarifyTests(unittest.TestCase):
    def test_area_conservation_and_bounds(self):
        areas = [300.0, 200.0, 150.0, 100.0, 90.0, 60.0, 40.0]
        total = sum(areas)
        canvas_w, canvas_h = 200.0, 150.0
        norm = [a * canvas_w * canvas_h / total for a in areas]
        rects = _squarify(norm, 0.0, 0.0, canvas_w, canvas_h)
        self.assertEqual(len(rects), len(areas))
        for x, y, w, h in rects:
            self.assertGreaterEqual(x, -1e-6)
            self.assertGreaterEqual(y, -1e-6)
            self.assertLessEqual(x + w, canvas_w + 1e-6)
            self.assertLessEqual(y + h, canvas_h + 1e-6)
        self.assertAlmostEqual(sum(w * h for _, _, w, h in rects), canvas_w * canvas_h, places=4)

    def test_reasonable_aspect_ratios(self):
        areas = [float(3000 - i * i * 7) for i in range(60)]
        total = sum(areas)
        norm = [a * 1000 * 600 / total for a in areas]
        rects = _squarify(norm, 0.0, 0.0, 1000.0, 600.0)
        worst = max(max(w / h, h / w) for _, _, w, h in rects)
        self.assertLess(worst, 8.0, f"worst aspect ratio {worst:.1f} too thin")

    def test_deterministic(self):
        areas = [9.0, 5.0, 3.0, 1.0]
        a = _squarify(list(areas), 0, 0, 18, 10)
        b = _squarify(list(areas), 0, 0, 18, 10)
        self.assertEqual(a, b)

    def test_single_module_fills(self):
        rects = _squarify([600.0], 0, 0, 30, 20)
        self.assertEqual(rects, [(0.0, 0.0, 30.0, 20.0)])


class TreemapSceneTests(unittest.TestCase):
    def test_every_module_has_a_cell_grid_and_border(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config(width=1000, treemap_height=700))
        grids = _cells(scene)
        self.assertEqual({g.module for g in grids}, {"DllBag", "DllBattle", "DllTitle"})
        borders = [r for r in _rects(scene) if r.stroke and r.stroke_width >= 1.5]
        self.assertGreaterEqual(len(borders), 3, "expected strong module borders")

    def test_cells_carry_hover_metadata(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config(width=1000, treemap_height=700))
        g = next(c for c in _cells(scene) if c.module == "DllTitle")
        self.assertEqual(g.base_addr, 0x180)
        self.assertEqual(len(g.tiers), 3)
        self.assertIn(("data-module", "DllTitle"), g.attrs)

    def test_header_and_legend_present(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config())
        texts = [i.s for i in scene.items if not isinstance(i, (Rect, Cells))]
        self.assertTrue(any("test corpus" in t for t in texts))
        self.assertTrue(any("interpreter hits" in t for t in texts))

    def test_hybrid_minor_cards_hold_all_cells(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config())  # default minor_max=200: all sample modules are minors
        grids = _cells(scene)
        counts = {g.module: len(g.tiers) for g in grids}
        self.assertEqual(counts, {"DllBag": 3, "DllBattle": 40, "DllTitle": 3})
        self.assertTrue(any("fixed cards" in t for t in (i.s for i in scene.items if not isinstance(i, (Rect, Cells)))))

    def test_hybrid_split_major_minor(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config(width=1000, treemap_height=500), minor_max=10)
        grids = {g.module: g for g in _cells(scene)}
        # DllBattle (40 fns > minor_max) is the major: its squarified box is
        # far larger than a fixed minor card
        self.assertGreater(grids["DllBattle"].w, 300)
        # DllBag/DllTitle (3 fns) are minor cards: every cell drawn
        self.assertEqual(len(grids["DllBag"].tiers), 3)
        self.assertEqual(len(grids["DllTitle"].tiers), 3)
        self.assertLessEqual(grids["DllBag"].w, 160)

    def test_cells_stay_inside_box_borders(self):
        ds = sample_dataset()
        scene = treemap_scene(ds, Config(width=1200, treemap_height=700), minor_max=10)
        borders = [
            r for r in _rects(scene)
            if r.stroke and r.stroke_width >= 1.5
        ]
        for g in _cells(scene):
            inside = [
                b for b in borders
                if b.x - 0.5 <= g.x and b.y - 0.5 <= g.y
                and g.x + g.w <= b.x + b.w + 0.5
                and g.y + g.h <= b.y + b.h + 0.5
            ]
            self.assertTrue(inside, f"cells of {g.module} overflow every candidate border")


class GridSceneTests(unittest.TestCase):
    def test_module_grid_renders_all_cells_in_address_order(self):
        ds = sample_dataset()
        cfg = Config()
        scene = grid_scene(ds, "DllTitle", cfg)
        grids = _cells(scene)
        self.assertEqual(len(grids), 1)
        g = grids[0]
        self.assertEqual(g.tiers, (0, 1, 2))  # raw, decompiled, named in address order
        self.assertEqual(g.base_addr, 0x180)
        self.assertEqual(g.cols, cfg.grid_cols)

    def test_unknown_module_raises(self):
        with self.assertRaises(HeatmapError):
            grid_scene(sample_dataset(), "DllNope", Config())

    def test_canvas_fits_grid(self):
        ds = sample_dataset()
        cfg = Config(width=500, grid_cols=8)
        scene = grid_scene(ds, "DllBattle", cfg)  # 40 cells -> 5 rows
        g = _cells(scene)[0]
        self.assertEqual(g.rows, 5)
        self.assertGreaterEqual(scene.width, g.w + 2 * cfg.margin)

    def test_min_tier_filters_cells(self):
        ds = sample_dataset()
        scene = grid_scene(ds, "DllTitle", Config(), min_tier=2)
        g = _cells(scene)[0]
        self.assertEqual(g.tiers, (2,))  # only the named cell survives
        self.assertEqual(g.base_addr, 0x188)


class CorpusGridTests(unittest.TestCase):
    def test_section_per_module(self):
        ds = sample_dataset()
        scene = corpus_grid_scene(ds, Config(width=900))
        grids = _cells(scene)
        self.assertEqual({g.module for g in grids}, {"DllBag", "DllBattle", "DllTitle"})

    def test_min_tier(self):
        ds = sample_dataset()
        scene = corpus_grid_scene(ds, Config(width=900), min_tier=3)
        grids = _cells(scene)
        totals = {g.module: len(g.tiers) for g in grids}
        # DllTitle has no verified cells -> no grid section at all
        self.assertEqual(totals, {"DllBag": 1, "DllBattle": 10})


class BarViewTests(unittest.TestCase):
    def test_overview_row_per_module_sorted_by_count(self):
        ds = sample_dataset()
        scene = overview_scene(ds, Config())
        texts = [i.s for i in scene.items if not isinstance(i, (Rect, Cells))]
        self.assertIn("DllBattle", texts)
        self.assertIn("DllBag", texts)

    def test_namespaces_explain_themselves(self):
        ds = sample_dataset()
        scene = namespaces_scene(ds, Config())
        texts = [i.s for i in scene.items if not isinstance(i, (Rect, Cells))]
        joined = "\n".join(texts)
        self.assertIn("ui::title", joined)
        self.assertIn("in DllTitle", joined)  # module attribution
        self.assertIn("(unnamed — no namespace yet)", joined)
        self.assertIn("subsystems visible in named symbols", joined)

    def test_fit_truncates(self):
        self.assertEqual(_fit("short", 100, 12), "short")
        self.assertTrue(_fit("x" * 200, 50, 12).endswith("…"))
        self.assertLessEqual(len(_fit("x" * 200, 50, 12)), 8)


if __name__ == "__main__":
    unittest.main()
