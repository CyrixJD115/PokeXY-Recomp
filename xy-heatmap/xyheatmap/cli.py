"""The xy-heatmap command line — the package's one entry point.

    xy-heatmap serve [--watch]                 interactive local web UI
    xy-heatmap export --view treemap --format png --out treemap.png --scale 3
    xy-heatmap summary                         text table to stdout
    xy-heatmap readme [--out docs/assets]      regenerate the README assets
    xy-heatmap selftest                        run the bundled test suite

Data comes from the project corpus (auto-discovered .Data/Source_Extracted,
or --data-root / $XY_HEATMAP_DATA_ROOT) or from a generic --dataset JSON
file; see README.md for the schema.
"""

from __future__ import annotations

import argparse
import os
import sys
import unittest
from pathlib import Path

from xyheatmap import __version__
from xyheatmap.collect import (
    FileDatasetSource,
    ProjectSource,
    discover_data_root,
)
from xyheatmap.config import Config
from xyheatmap.errors import HeatmapError
from xyheatmap.png import png_bytes, png_info
from xyheatmap.svg import render_svg
from xyheatmap.views import VIEW_NAMES, build_scene, parse_view, view_filename


def _add_source_args(p: argparse.ArgumentParser) -> None:
    p.add_argument("--data-root", metavar="DIR", help="Source_Extracted root (default: auto-discover)")
    p.add_argument("--dataset", metavar="FILE", help="use a generic dataset JSON file instead of the corpus")


def _add_render_args(p: argparse.ArgumentParser) -> None:
    p.add_argument("--width", type=int, default=None, help="canvas width in scene px (default 1280)")


def _source(args) -> object:
    if args.dataset:
        return FileDatasetSource(Path(args.dataset))
    if args.data_root:
        return ProjectSource(Path(args.data_root))
    env = os.environ.get("XY_HEATMAP_DATA_ROOT")
    if env:
        return ProjectSource(Path(env))
    return ProjectSource(discover_data_root())


def _config(args) -> Config:
    cfg = Config()
    if getattr(args, "width", None):
        cfg.width = args.width
    return cfg


def _dataset(args):
    return _source(args).dataset()


# ----------------------------------------------------------------- commands


def cmd_serve(args) -> int:
    from xyheatmap.server import DatasetCache, serve

    app = DatasetCache(_source(args))
    serve(app, _config(args), args.host, args.port, args.watch, args.open)
    return 0


def cmd_export(args) -> int:
    ds = _dataset(args)
    cfg = _config(args)
    fmt = args.format
    out = Path(args.out)
    min_tier = max(0, min(3, args.min_tier))

    targets: list[tuple[str, str | None, Path]] = []
    if args.view == "all":
        if out.suffix:
            raise HeatmapError("--out must be a directory when --view all")
        for name in VIEW_NAMES:
            targets.append((name, None, out / view_filename(name, None, fmt)))
    else:
        view, module = parse_view(args.view)
        targets.append((view, module, out))

    written = 0
    for view, module, path in targets:
        cols = args.cols or None
        scene = build_scene(ds, view, module, cfg, min_tier, cols, minor_max=args.minor_max)
        path.parent.mkdir(parents=True, exist_ok=True)
        if fmt == "svg":
            data = render_svg(scene)
            path.write_bytes(data)
            kb = len(data) / 1024
            print(f"wrote {path} ({int(scene.width)}×{int(scene.height)}, {kb:.1f} KB)")
        else:
            warnings: list[str] = []
            data = png_bytes(scene, scale=args.scale, dpi=args.dpi, warnings=warnings)
            path.write_bytes(data)
            w, h, dpi = png_info(scene, args.scale, args.dpi)
            kb = len(data) / 1024
            print(f"wrote {path} ({w}×{h} px @ dpi {dpi}, {kb:.1f} KB)")
            if warnings and not args.quiet:
                for warn in warnings:
                    print(f"note: {warn}", file=sys.stderr)
        written += 1
    if not args.quiet:
        print(f"{written} file{'s' if written != 1 else ''} written.")
    return 0


def cmd_summary(args) -> int:
    ds = _dataset(args)
    tiers = ["raw", "decompiled", "named", "verified"]
    modules = list(ds.modules)
    modules.sort(key=lambda m: m.name if args.sort == "name" else (-len(m.cells), m.name))

    name_w = max([len(m.name) for m in modules] + [len("module")]) + 2
    header = (
        f"{'module':<{name_w}}{'fns':>8}{'covered':>9}{'decompiled':>11}"
        f"{'named':>8}{'verified':>9}  relink"
    )
    print(header)
    print("-" * len(header))
    for m in modules:
        total = len(m.cells)
        counts = m.tier_counts()
        relink = "-" if m.relink_ok is None else ("ok" if m.relink_ok else "MISMATCH")
        print(
            f"{m.name:<{name_w}}{total:>8,}"
            f"{_pct(m.covered_count(), total):>9}"
            f"{_pct(counts['decompiled'], total):>11}"
            f"{_pct(counts['named'], total):>8}"
            f"{_pct(counts['verified'], total):>9}  {relink}"
        )
    total = ds.total_cells()
    counts = ds.tier_counts()
    relink_s = ""
    relink = ds.relink_summary()
    if relink:
        relink_s = f"  {relink[0]}/{relink[1]}"
    print("-" * len(header))
    print(
        f"{'TOTAL':<{name_w}}{total:>8,}"
        f"{_pct(ds.covered_count(), total):>9}"
        f"{_pct(counts['decompiled'], total):>11}"
        f"{_pct(counts['named'], total):>8}"
        f"{_pct(counts['verified'], total):>9}{relink_s}"
    )
    if ds.warnings and not args.quiet:
        print(f"\n{len(ds.warnings)} collector warning(s):", file=sys.stderr)
        for w in ds.warnings[:8]:
            print(f"  - {w}", file=sys.stderr)
        if len(ds.warnings) > 8:
            print(f"  … and {len(ds.warnings) - 8} more", file=sys.stderr)
    return 0


def cmd_readme(args) -> int:
    cfg = _config(args)
    if args.out:
        out = Path(args.out)
    else:
        # default: <repo>/docs/assets next to the discovered data root
        source = _source(args)
        root = getattr(source, "root", None)
        out = root.parent.parent / "docs" / "assets" if root is not None else Path("docs") / "assets"
    out.mkdir(parents=True, exist_ok=True)
    rc = cmd_export(
        _Args(view="treemap", format="svg", out=str(out / "heatmap-treemap.svg"),
              scale=2, dpi=None, width=cfg.width, quiet=args.quiet, dataset=args.dataset,
              data_root=args.data_root)
    )
    rc += cmd_export(
        _Args(view="overview", format="svg", out=str(out / "heatmap-progress.svg"),
              scale=2, dpi=None, width=cfg.width, quiet=args.quiet, dataset=args.dataset,
              data_root=args.data_root)
    )
    return 0 if rc == 0 else 1


def cmd_selftest(args) -> int:
    tests_dir = Path(__file__).parent / "tests"
    suite = unittest.defaultTestLoader.discover(str(tests_dir), top_level_dir=str(tests_dir.parent))
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


class _Args:
    """Tiny attribute bag so readme can reuse cmd_export."""

    def __init__(self, **kw) -> None:
        self.quiet = False
        self.min_tier = 0
        self.cols = None
        self.minor_max = None
        for k, v in kw.items():
            setattr(self, k, v)


def _pct(n: int, d: int) -> str:
    return f"{(100.0 * n / d):.1f}%" if d else "0.0%"


# ------------------------------------------------------------------- parser


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="xy-heatmap",
        description="Standalone progress-heatmap studio: local web UI + SVG/PNG export.",
    )
    p.add_argument("--version", action="version", version=f"xy-heatmap {__version__}")
    sub = p.add_subparsers(dest="command", required=True)

    s = sub.add_parser("serve", help="host the interactive web UI (writes nothing to disk)")
    _add_source_args(s)
    s.add_argument("--host", default="127.0.0.1")
    s.add_argument("--port", type=int, default=8765)
    s.add_argument("--watch", action="store_true", help="auto-reload when the sources change")
    s.add_argument("--open", action="store_true", help="open the UI in the default browser")
    s.set_defaults(func=cmd_serve)

    e = sub.add_parser("export", help="write SVG/PNG exports directly")
    _add_source_args(e)
    _add_render_args(e)
    e.add_argument("--view", required=True,
                   help="treemap | overview | grid | namespaces | module:<NAME> | all")
    e.add_argument("--format", choices=("svg", "png"), default="svg")
    e.add_argument("--out", required=True, help="output file (or directory with --view all)")
    e.add_argument("--scale", type=int, default=2, help="PNG px per scene unit (default 2)")
    e.add_argument("--dpi", type=int, default=None, help="PNG DPI metadata (default 96×scale)")
    e.add_argument("--min-tier", type=int, default=0, choices=(0, 1, 2, 3),
                   help="only show cells at this tier or above (0 raw..3 verified)")
    e.add_argument("--cols", type=int, default=None,
                   help="grid columns for grid/module views (default from config)")
    e.add_argument("--minor-max", type=int, default=None,
                   help="treemap hybrid: modules up to this many functions become fixed cards (0 = pure treemap)")
    e.add_argument("--quiet", action="store_true")
    e.set_defaults(func=cmd_export)

    t = sub.add_parser("summary", help="per-module text table to stdout")
    _add_source_args(t)
    t.add_argument("--sort", choices=("name", "count"), default="name")
    t.add_argument("--quiet", action="store_true")
    t.set_defaults(func=cmd_summary)

    r = sub.add_parser("readme", help="regenerate the README heatmap assets (treemap + progress)")
    _add_source_args(r)
    _add_render_args(r)
    r.add_argument("--out", default=None, help="target directory (default <repo>/docs/assets)")
    r.add_argument("--quiet", action="store_true")
    r.set_defaults(func=cmd_readme)

    st = sub.add_parser("selftest", help="run the bundled test suite")
    st.set_defaults(func=cmd_selftest)
    return p


def main(argv=None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except HeatmapError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
