"""PNG backend: geometry, DPI metadata, determinism, font fallback."""

import unittest

from xyheatmap.config import Config
from xyheatmap.layout import treemap_scene
from xyheatmap.png import discover_fonts, png_bytes, render_png
from xyheatmap.scene import Rect, Scene, Text

from .util import sample_dataset

try:
    from PIL import Image  # noqa: F401

    HAVE_PIL = True
except ImportError:  # pragma: no cover
    HAVE_PIL = False


@unittest.skipUnless(HAVE_PIL, "Pillow not installed")
class PngTests(unittest.TestCase):
    def test_size_and_mode_at_scale(self):
        scene = Scene(320, 200, "#121419")
        img = render_png(scene, scale=3)
        self.assertEqual(img.size, (960, 600))
        self.assertEqual(img.mode, "RGB")

    def test_dpi_metadata(self):
        scene = Scene(320, 200, "#121419")
        img = render_png(scene, scale=2, dpi=300)
        self.assertEqual(img.info["dpi"], (300, 300))
        img2 = render_png(scene, scale=4)
        self.assertEqual(img2.info["dpi"], (384, 384))

    def test_deterministic_bytes(self):
        ds = sample_dataset()
        cfg = Config(width=800)
        a = png_bytes(treemap_scene(ds, cfg), scale=2)
        b = png_bytes(treemap_scene(ds, cfg), scale=2)
        self.assertEqual(a, b)
        self.assertEqual(a[:8], b"\x89PNG\r\n\x1a\n")

    def test_not_blank(self):
        ds = sample_dataset()
        import io

        from PIL import Image

        img = Image.open(io.BytesIO(png_bytes(treemap_scene(ds, Config()), scale=2)))
        self.assertEqual(img.getextrema() != ((0, 0), (0, 0), (0, 0)), True)

    def test_transparent_hit_target_skipped(self):
        scene = Scene(50, 50, "#101010")
        scene.add(Rect(0, 0, 50, 50, "#00000000"))
        img = render_png(scene, scale=1)
        self.assertEqual(img.getpixel((25, 25)), (0x10, 0x10, 0x10))

    def test_renders_text_without_crash(self):
        scene = Scene(200, 60, "#121419")
        scene.add(Text(10, 30, "DllBattle — 3,011", 12))
        scene.add(Text(190, 45, "end", 10, anchor="end", weight="bold"))
        regular, bold = discover_fonts()
        img = render_png(scene, scale=2)
        self.assertEqual(img.size, (400, 120))
        # text pixels exist in the text region (font present) or at least no crash
        if regular:
            region = img.crop((16, 40, 300, 70))
            lo, hi = region.convert("L").getextrema()
            self.assertLess(lo, hi, "expected visible glyphs")

    def test_cells_are_crisp_at_integral_scale(self):
        scene = Scene(40, 10, "#000000")
        scene.add(Rect(0, 0, 10, 10, "#ffffff"))
        img = render_png(scene, scale=1)
        self.assertEqual(img.getpixel((5, 5)), (255, 255, 255))
        self.assertEqual(img.getpixel((15, 5)), (0, 0, 0))


if __name__ == "__main__":
    unittest.main()
