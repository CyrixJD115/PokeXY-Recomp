"""The local web UI host — stdlib http.server, zero files written.

Serves from memory only: the page string, a compact dataset JSON payload
(gzipped when the browser offers it), a cheap fingerprint endpoint for live
reload, and the /view/*.svg|*.png scenes rendered on demand by the very same
layout + renderer code the CLI exporter uses.

Run: ``xy-heatmap serve --watch --port 8765`` then open the printed URL.
Ctrl-C stops it cleanly; nothing anywhere on disk was created.
"""

from __future__ import annotations

import gzip
import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, unquote, urlparse

from xyheatmap import __version__
from xyheatmap.dataset import Dataset, dataset_to_api
from xyheatmap.errors import HeatmapError
from xyheatmap.png import png_bytes
from xyheatmap.views import VIEW_NAMES, build_scene
from xyheatmap.webui import PAGE


class DatasetCache:
    """Fingerprint-gated dataset cache shared across request threads."""

    def __init__(self, source) -> None:
        self.source = source
        self._lock = threading.Lock()
        self._fp: str | None = None
        self._ds: Dataset | None = None

    def fingerprint(self) -> str:
        with self._lock:
            return self.source.fingerprint()

    def dataset(self) -> Dataset:
        with self._lock:
            fp = self.source.fingerprint()
            if self._ds is None or fp != self._fp:
                self._ds = self.source.dataset()
                self._ds.fingerprint = fp
                self._fp = fp
            return self._ds


def make_handler(app: DatasetCache, watch: bool, config) -> type:
    class Handler(BaseHTTPRequestHandler):
        server_version = f"xy-heatmap/{__version__}"
        protocol_version = "HTTP/1.1"

        # -- helpers ------------------------------------------------------
        def _send(self, code: int, body: bytes, ctype: str, gzip_ok: bool = False) -> None:
            encodings = self.headers.get("Accept-Encoding", "")
            if (
                gzip_ok
                and "gzip" in encodings
                and len(body) > 1024
                and not ctype.startswith("image/")
            ):
                body = gzip.compress(body)
                self.send_response(code)
                self.send_header("Content-Type", ctype + "; charset=utf-8")
                self.send_header("Content-Encoding", "gzip")
            else:
                self.send_response(code)
                if ctype.startswith(("text/", "application/json", "image/svg")):
                    ctype += "; charset=utf-8"
                self.send_header("Content-Type", ctype)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def _json(self, code: int, obj) -> None:
            self._send(code, json.dumps(obj).encode(), "application/json", gzip_ok=True)

        def _fail(self, code: int, message: str) -> None:
            if self.path.startswith(("/api", "/view")):
                self._json(code, {"error": message})
            else:
                self._send(code, message.encode(), "text/plain")

        def log_message(self, fmt, *args):  # keep the console clean
            pass

        # -- routes -------------------------------------------------------
        def do_GET(self) -> None:  # noqa: N802 (stdlib naming)
            try:
                url = urlparse(self.path)
                path, qs = url.path, parse_qs(url.query)
                if path in ("/", "/index.html"):
                    page = (
                        PAGE.replace("__WATCH__", "true" if watch else "false")
                        .replace("__SOURCES__", app.source.describe())
                    )
                    self._send(200, page.encode(), "text/html")
                elif path == "/favicon.ico":
                    self.send_response(204)
                    self.send_header("Content-Length", "0")
                    self.end_headers()
                elif path == "/api/dataset":
                    ds = app.dataset()
                    payload = dataset_to_api(ds)
                    payload["warnings"] = ds.warnings
                    payload["watch"] = watch
                    self._json(200, payload)
                elif path == "/api/fingerprint":
                    self._json(200, {"fingerprint": app.fingerprint()})
                elif path.startswith("/api/module/"):
                    self._module_api(unquote(path[len("/api/module/") :]))
                elif path.startswith("/view/"):
                    self._view(path[len("/view/") :], qs)
                else:
                    self._fail(404, f"not found: {path}")
            except (BrokenPipeError, ConnectionResetError):
                pass  # browser cancelled a download; nothing to do

        @staticmethod
        def _int_param(qs: dict, key: str, default: int, lo: int, hi: int) -> int:
            try:
                return min(hi, max(lo, int(qs.get(key, [default])[0])))
            except (ValueError, TypeError):
                return default

        def _module_api(self, name: str) -> None:
            try:
                mod = app.dataset().module(name)
            except HeatmapError as e:
                self._fail(404, str(e))
                return
            counts = mod.tier_counts()
            addrs = [c.addr for c in mod.cells]
            named = sum(1 for c in mod.cells if c.name)
            self._json(
                200,
                {
                    "name": mod.name,
                    "total": len(mod.cells),
                    "tiers": counts,
                    "covered": mod.covered_count(),
                    "named_symbols": named,
                    "relink_ok": mod.relink_ok,
                    "addr_range": (
                        [f"0x{addrs[0]:X}", f"0x{addrs[-1]:X}"] if addrs else None
                    ),
                    "imports": list(mod.imports),
                },
            )

        def _view(self, spec: str, qs: dict) -> None:
            base, dot, ext = spec.rpartition(".")
            if not dot or ext not in ("svg", "png") or not base:
                self._fail(404, f"bad view path {spec!r}")
                return
            view, module = None, None
            if base.startswith("module-"):
                view, module = "module", unquote(base[len("module-") :])
            elif base in VIEW_NAMES:
                view = base
            if view is None:
                self._fail(404, f"unknown view {base!r}")
                return
            min_tier = self._int_param(qs, "min", 0, 0, 3)
            cols = self._int_param(qs, "cols", 0, 16, 256) or None
            bare = qs.get("bare", ["0"])[0] == "1"
            minor_max = self._int_param(qs, "minor", -1, 0, 100000)
            try:
                scene = build_scene(
                    app.dataset(), view, module, config, min_tier, cols, bare,
                    minor_max=None if minor_max < 0 else minor_max,
                )
            except HeatmapError as e:
                self._fail(404, str(e))
                return
            if ext == "svg":
                from xyheatmap.svg import render_svg

                self._send(200, render_svg(scene), "image/svg+xml")
            else:
                try:
                    scale = min(8, max(1, int(qs.get("scale", ["2"])[0])))
                except ValueError:
                    scale = 2
                dpi = None
                if "dpi" in qs:
                    try:
                        dpi = min(1200, max(36, int(qs["dpi"][0])))
                    except ValueError:
                        dpi = None
                self._send(200, png_bytes(scene, scale=scale, dpi=dpi), "image/png")

        def version_string(self) -> str:
            return self.server_version

    return Handler


def serve(app: DatasetCache, config, host: str, port: int, watch: bool, open_browser: bool = False) -> None:
    httpd = ThreadingHTTPServer((host, port), make_handler(app, watch, config))
    url = f"http://{host}:{httpd.server_address[1]}/"
    print(f"xy-heatmap {__version__} serving {app.source.describe()}")
    print(f"  -> {url}   (Ctrl-C to stop; nothing is written to disk)")
    if open_browser:
        import webbrowser

        webbrowser.open(url)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        httpd.server_close()
        print("stopped.")
