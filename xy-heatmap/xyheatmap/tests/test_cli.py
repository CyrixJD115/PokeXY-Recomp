"""CLI tests: export/summary/readme flows against a generic dataset file."""

import io
import json
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

from xyheatmap.cli import main

from .util import sample_dataset_dict


class CliTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.td = Path(self._tmp.name)
        self.ds = self.td / "ds.json"
        self.ds.write_text(json.dumps(sample_dataset_dict()))

    def tearDown(self):
        self._tmp.cleanup()

    def _run(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            rc = main(list(argv))
        return rc, out.getvalue(), err.getvalue()

    def test_export_svg(self):
        out_path = self.td / "treemap.svg"
        rc, stdout, _ = self._run(
            "export", "--dataset", str(self.ds), "--view", "treemap",
            "--format", "svg", "--out", str(out_path),
        )
        self.assertEqual(rc, 0)
        self.assertTrue(out_path.is_file())
        self.assertIn("wrote", stdout)
        self.assertIn("1280×", stdout)  # default canvas width

    def test_export_png_scale(self):
        out_path = self.td / "grid.png"
        rc, stdout, _ = self._run(
            "export", "--dataset", str(self.ds), "--view", "module:DllBag",
            "--format", "png", "--out", str(out_path), "--scale", "3",
        )
        self.assertEqual(rc, 0)
        self.assertTrue(out_path.is_file())
        self.assertGreater(out_path.stat().st_size, 200)
        self.assertIn("@ dpi 288", stdout)

    def test_export_all_to_directory(self):
        outdir = self.td / "assets"
        rc, _, _ = self._run(
            "export", "--dataset", str(self.ds), "--view", "all",
            "--format", "svg", "--out", str(outdir),
        )
        self.assertEqual(rc, 0)
        names = {p.name for p in outdir.iterdir()}
        self.assertEqual(names, {"treemap.svg", "overview.svg", "grid.svg", "namespaces.svg"})

    def test_export_unknown_view_fails_cleanly(self):
        rc, _, err = self._run(
            "export", "--dataset", str(self.ds), "--view", "vibes",
            "--format", "svg", "--out", str(self.td / "x.svg"),
        )
        self.assertEqual(rc, 1)
        self.assertIn("unknown view", err)

    def test_export_all_rejects_file_out(self):
        rc, _, err = self._run(
            "export", "--dataset", str(self.ds), "--view", "all",
            "--format", "svg", "--out", str(self.td / "file.svg"),
        )
        self.assertEqual(rc, 1)
        self.assertIn("must be a directory", err)

    def test_summary_table(self):
        rc, stdout, _ = self._run("summary", "--dataset", str(self.ds))
        self.assertEqual(rc, 0)
        self.assertIn("DllBattle", stdout)
        self.assertIn("TOTAL", stdout)
        self.assertIn("1/2", stdout)  # relink summary

    def test_readme_assets(self):
        outdir = self.td / "docs" / "assets"
        rc, _, _ = self._run("readme", "--dataset", str(self.ds), "--out", str(outdir))
        self.assertEqual(rc, 0)
        self.assertTrue((outdir / "heatmap-treemap.svg").is_file())
        self.assertTrue((outdir / "heatmap-progress.svg").is_file())

    def test_module_export_unknown_module(self):
        rc, _, err = self._run(
            "export", "--dataset", str(self.ds), "--view", "module:Ghost",
            "--format", "svg", "--out", str(self.td / "g.svg"),
        )
        self.assertEqual(rc, 1)
        self.assertIn("unknown module", err)


if __name__ == "__main__":
    unittest.main()
