"""--watch (regenerate on input change) and --serve (live page over http).

serve() starts a ThreadingHTTPServer on the output directory; when a
watch_args namespace is passed it also runs the fingerprint loop in a
daemon thread, rebuilding the dashboard whenever any collector input
changes. The page itself polls heatmap_data.json every 3 s when loaded
over http(s), so watch + serve gives a fully live dashboard.
"""
import http.server
import functools
import sys
import threading
import time

from . import collect
from . import page


def _watch_loop(args, sources):
    last = sources.fingerprint_inputs()
    while True:
        time.sleep(2)
        cur = sources.fingerprint_inputs()
        if cur != last:
            last = cur
            try:
                from .cli import rebuild
                rebuild(args)
            except Exception as e:  # keep serving on regen errors
                print(f"watch: rebuild failed: {e}", file=sys.stderr)


def serve(args, out_dir, port, watch=False):
    if watch:
        threading.Thread(
            target=_watch_loop, args=(args, collect.Sources(args)),
            daemon=True).start()
    handler = functools.partial(
        http.server.SimpleHTTPRequestHandler, directory=out_dir)
    url = f"http://127.0.0.1:{port}/heatmap.html"
    print(f"serving {out_dir} -> {url} (Ctrl-C to stop)")
    http.server.ThreadingHTTPServer(("127.0.0.1", port), handler).serve_forever()
