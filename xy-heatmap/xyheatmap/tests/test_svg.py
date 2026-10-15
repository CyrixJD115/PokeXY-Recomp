"""SVG backend: well-formedness, determinism, escaping, native titles."""

import unittest
import xml.etree.ElementTree as ET

from xyheatmap.config import Config
from xyheatmap.layout import overview_scene, treemap_scene
from xyheatmap.scene import Line, Rect, Scene, Text
from xyheatmap.svg import render_svg

from .util import sample_dataset


def _root(scene: Scene) -> ET.Element:
    return ET.fromstring(render_svg(scene).decode("utf-8"))


class SvgTests(unittest.TestCase):
    def test_wellformed_with_namespace(self):
        root = _root(Scene(100, 50, "#121419"))
        self.assertEqual(root.tag, "{http://www.w3.org/2000/svg}svg")
        self.assertEqual(root.get("viewBox"), "0 0 100 50")
        self.assertEqual(root.get("width"), "100")

    def test_rect_and_line_counts(self):
        scene = Scene(100, 50, "#000")
        scene.add(Rect(0, 0, 10, 10, "#454b57"))
        scene.add(Rect(5, 5, 10, 10, "#58b368", tip="hello"))
        scene.add(Line(0, 0, 99, 0, "#fff"))
        svg = render_svg(scene).decode()
        root = ET.fromstring(svg)
        ns = "{http://www.w3.org/2000/svg}"
        self.assertEqual(len(root.findall(f"{ns}rect")), 3)  # + background
        self.assertEqual(len(root.findall(f"{ns}line")), 1)
        self.assertIn("<title>hello</title>", svg)

    def test_text_escaping(self):
        scene = Scene(100, 50, "#000")
        scene.add(Text(1, 10, '<Dll&"Bag">', 12))
        svg = render_svg(scene).decode()
        # quotes are legal inside element content; & and < must be escaped
        self.assertIn("&lt;Dll&amp;\"Bag\"&gt;", svg)
        self.assertNotIn("<Dll", svg)

    def test_text_anchor_and_weight(self):
        scene = Scene(100, 50, "#000")
        scene.add(Text(50, 10, "mid", 12, anchor="middle", weight="bold"))
        svg = render_svg(scene).decode()
        self.assertIn('text-anchor="middle"', svg)
        self.assertIn('font-weight="bold"', svg)

    def test_deterministic_bytes(self):
        ds = sample_dataset()
        a = render_svg(treemap_scene(ds, Config()))
        b = render_svg(treemap_scene(ds, Config()))
        self.assertEqual(a, b)

    def test_real_views_render(self):
        ds = sample_dataset()
        for scene in (treemap_scene(ds, Config()), overview_scene(ds, Config())):
            root = ET.fromstring(render_svg(scene).decode())
            self.assertEqual(root.tag, "{http://www.w3.org/2000/svg}svg")

    def test_numbers_compact(self):
        scene = Scene(10.0, 5.0, "#000")
        scene.add(Rect(1.0, 2.0, 3.0, 4.0, "#fff"))
        svg = render_svg(scene).decode()
        # the last <rect> is the data rect; integral floats must not print ".0"
        rect = "<rect" + svg.rsplit("<rect", 1)[1]
        self.assertIn('x="1" y="2" width="3" height="4"', rect)
        self.assertNotIn(".0", rect)

    def test_data_attrs(self):
        scene = Scene(100, 50, "#000")
        scene.add(Rect(0, 0, 5, 5, "#fff", attrs=(("data-module", "DllBag"),)))
        svg = render_svg(scene).decode()
        self.assertIn('data-module="DllBag"', svg)


if __name__ == "__main__":
    unittest.main()
