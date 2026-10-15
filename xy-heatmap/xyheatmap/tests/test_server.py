"""End-to-end server tests: routes, content types, gzip, zero disk writes."""

import json
import tempfile
import threading
import unittest
import urllib.error
import urllib.request
from pathlib import Path

from xyheatmap.config import Config
from xyheatmap.server import DatasetCache, make_handler
from xyheatmap.collect import FileDatasetSource

from .util import sample_dataset_dict


def _get(url, headers=None):
    req = urllib.request.Request(url, headers=headers or {})
    try:
        with urllib.request.urlopen(req) as r:
            return r.status, dict(r.headers), r.read()
    except urllib.error.HTTPError as e:
        with e:
            return e.code, dict(e.headers), e.read()


class ServerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls._tmp = tempfile.TemporaryDirectory()
        td = Path(cls._tmp.name)
        cls.ds_path = td / "ds.json"
        cls.ds_path.write_text(json.dumps(sample_dataset_dict()))
        app = DatasetCache(FileDatasetSource(cls.ds_path))
        httpd_type = make_handler(app, watch=False, config=Config(width=800, treemap_height=600))
        from http.server import ThreadingHTTPServer

        cls.httpd = ThreadingHTTPServer(("127.0.0.1", 0), httpd_type)
        cls.port = cls.httpd.server_address[1]
        cls.thread = threading.Thread(target=cls.httpd.serve_forever, daemon=True)
        cls.thread.start()
        cls.base = f"http://127.0.0.1:{cls.port}"

    @classmethod
    def tearDownClass(cls):
        cls.httpd.shutdown()
        cls.httpd.server_close()
        cls._tmp.cleanup()

    def test_page_served_in_memory(self):
        status, headers, body = _get(self.base + "/")
        self.assertEqual(status, 200)
        self.assertTrue(headers["Content-Type"].startswith("text/html"))
        self.assertIn(b"xy-heatmap", body)
        self.assertNotIn(b"__WATCH__", body)  # placeholder was substituted

    def test_dataset_api(self):
        status, headers, body = _get(self.base + "/api/dataset")
        self.assertEqual(status, 200)
        data = json.loads(body)
        self.assertEqual([m["name"] for m in data["modules"]], ["DllBag", "DllBattle", "DllTitle"])
        self.assertTrue(data["fingerprint"])

    def test_fingerprint_api(self):
        status, _, body = _get(self.base + "/api/fingerprint")
        self.assertEqual(status, 200)
        self.assertTrue(json.loads(body)["fingerprint"])

    def test_svg_view(self):
        status, headers, body = _get(self.base + "/view/treemap.svg")
        self.assertEqual(status, 200)
        self.assertTrue(headers["Content-Type"].startswith("image/svg+xml"))
        self.assertIn(b"<svg", body)

    def test_png_view_with_scale(self):
        status, headers, body = _get(self.base + "/view/overview.png?scale=3")
        self.assertEqual(status, 200)
        self.assertEqual(headers["Content-Type"], "image/png")
        self.assertEqual(body[:8], b"\x89PNG\r\n\x1a\n")

    def test_module_view_and_unknown(self):
        status, _, body = _get(self.base + "/view/module-DllBag.svg")
        self.assertEqual(status, 200)
        self.assertIn(b"data-module", body)
        status, _, body = _get(self.base + "/view/module-Ghost.svg")
        self.assertEqual(status, 404)
        self.assertIn(b"unknown module", body)

    def test_module_facts_api(self):
        status, _, body = _get(self.base + "/api/module/DllBag")
        self.assertEqual(status, 200)
        import json as _json

        facts = _json.loads(body)
        self.assertEqual(facts["name"], "DllBag")
        self.assertEqual(facts["total"], 3)
        self.assertFalse(facts["relink_ok"])

    def test_min_tier_param(self):
        status, _, body = _get(self.base + "/view/module-DllTitle.svg?min=2")
        self.assertEqual(status, 200)
        status, _, body = _get(self.base + "/view/grid.svg?cols=48")
        self.assertEqual(status, 200)
        self.assertIn(b"<svg", body)

    def test_unknown_route(self):
        status, _, _ = _get(self.base + "/nope")
        self.assertEqual(status, 404)

    def test_gzip_when_offered(self):
        status, headers, body = _get(self.base + "/api/dataset", {"Accept-Encoding": "gzip"})
        self.assertEqual(status, 200)
        self.assertEqual(headers.get("Content-Encoding"), "gzip")
        import gzip as gz

        data = json.loads(gz.decompress(body))
        self.assertIn("modules", data)

    def test_no_files_written(self):
        listing = sorted(p.name for p in Path(self._tmp.name).iterdir())
        self.assertEqual(listing, ["ds.json"])


if __name__ == "__main__":
    unittest.main()
