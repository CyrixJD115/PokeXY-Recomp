"""Unified CLI for the heatmap toolkit.

    heatmap.py [flags] [build|serve|summary|roms]

  build    regenerate the dashboard (default when no command is given)
  serve    regenerate, then serve the page over http (--port, --watch)
  summary  print the per-module table to stdout
  roms     list ROMs found in .Data/Rom, or identify the one given via --rom

Flags (all optional; defaults anchor to the repo root wherever the CLI
is launched from):

  --rom PATH          identify this .3ds/.cxi (title ID -> X/Y/base/upd)
                      and use it to label the dashboard
  --source-root DIR   extraction tree (default .Data/Source_Extracted)
  --out DIR           output directory (default <src>/heatmap_gui)
  --recomp/--symbols/--cro/--decomp/--pipeline
                      per-source overrides for the collector
  --hints FILE        Zakuro interpreter-hits addresses (red overlay)
  --verified FILE     JSON [{module, addr}, …] of natively verified fns
  --port N            port for `serve` (default 8765)
  --watch             with `serve`: rebuild when inputs change
"""
import argparse
import sys

from . import DEFAULT_OUT, DEFAULT_SRC, ROOT, __version__
from . import collect, page, romdetect

COMMANDS = ("build", "serve", "summary", "roms", "readme")


def build_parser():
    ap = argparse.ArgumentParser(
        prog="heatmap.py",
        description="Pokémon X/Y recomp/decomp progress heatmap — "
                    "one central radial canvas + per-module grids, "
                    "PNG/SVG export, live serving.")
    ap.add_argument("--rom", metavar="PATH",
                    help="identify this .3ds/.cxi (title ID -> X/Y/base/"
                         "update) and label the dashboard with it")
    ap.add_argument("--source-root", metavar="DIR", default=DEFAULT_SRC,
                    help=f"extraction tree (default: {DEFAULT_SRC})")
    ap.add_argument("--out", metavar="DIR", default=DEFAULT_OUT,
                    help=f"output directory (default: {DEFAULT_OUT})")
    ap.add_argument("--recomp", help="3dsrecomp port dir override")
    ap.add_argument("--symbols", help="pipeline symbols dir override")
    ap.add_argument("--cro", help="cro_extracted root override")
    ap.add_argument("--decomp", help="decomp corpus root override")
    ap.add_argument("--pipeline", help="matching-decomp pipeline override")
    ap.add_argument("--hints", help="interpreter-hits address file "
                    "(red overlay)")
    ap.add_argument("--verified", help="JSON [{module, addr}, …] of "
                    "natively verified functions")
    ap.add_argument("--port", type=int, default=8765,
                    help="port for `serve` (default: 8765)")
    ap.add_argument("--assets-dir", metavar="DIR",
                    help="output dir for `readme` "
                         "(default: <repo>/docs/assets)")
    ap.add_argument("--watch", action="store_true",
                    help="with `serve`: rebuild when inputs change")
    ap.add_argument("--version", action="version",
                    version=f"%(prog)s {__version__}")
    ap.add_argument("command", nargs="?", choices=COMMANDS,
                    help="build (default) | serve | summary | roms | readme")
    return ap


def resolve_rom(args):
    """--rom path identification; None (with a warning) when unreadable."""
    if not args.rom:
        return None
    try:
        rom = romdetect.identify(args.rom)
    except (ValueError, OSError) as e:
        print(f"warning: --rom: {e}", file=sys.stderr)
        return None
    label = (f"{rom['game']} ({rom['kind']})" if rom["recognized"]
             else f"unrecognized title {rom['title_id']}")
    print(f"rom: {rom['title_id']} -> {label}")
    if rom["recognized"] and rom["game"] != "Pokémon X":
        print("note: data basis is the Pokémon X port; the Y dump only "
              "labels the dashboard for now", file=sys.stderr)
    return rom


def rebuild(args):
    """collect + treemap layout + write outputs; returns the data dict."""
    from . import assets
    rom = resolve_rom(args)
    data = collect.collect(args, rom=rom)
    data["treemap"] = assets.treemap_layout(data)
    page.build(data, args.out)
    return data


def cmd_readme(args):
    """Regenerate the committed README assets: refresh heatmap_data.json,
    then shell out to the xy-viz crate (charming/ECharts) to render the
    central treemap + progress bar SVGs, and print the markdown snippet."""
    import os
    import subprocess
    import sys

    rom = resolve_rom(args)
    data = collect.collect(args, rom=rom)
    from . import assets
    data["treemap"] = assets.treemap_layout(data)   # page canvas parity
    page.build(data, args.out)

    out_dir = args.assets_dir or os.path.join(ROOT, "docs", "assets")
    bin_path = os.path.join(ROOT, "target", "release", "xy-viz")
    if not os.path.exists(bin_path):
        print("building xy-viz (first time: compiles the charming "
              "renderer)…", file=sys.stderr)
        subprocess.run(["cargo", "build", "--release", "-p", "xy-viz"],
                       cwd=ROOT, check=True)
    subprocess.run(
        [bin_path, "--data", os.path.join(args.out, "heatmap_data.json"),
         "--out", out_dir], check=True)

    rel = os.path.relpath(out_dir, ROOT)
    print(f"\nREADME snippet:\n"
          f"![recomp/decomp progress]({rel}/heatmap-progress.svg)\n"
          f"![central treemap — every module a box]({rel}/"
          f"heatmap-treemap.svg)")
    return 0


def cmd_roms(args):
    roms = romdetect.scan_default_roms(ROOT)
    if args.rom:
        try:
            roms.insert(0, romdetect.identify(args.rom))
        except (ValueError, OSError) as e:
            print(f"error: {e}", file=sys.stderr)
            return 1
    if not roms:
        print(f"no identifiable ROMs under {ROOT}/.Data/Rom")
        return 0
    for r in roms:
        label = (f"{r['game']} · {r['kind']}" if r["recognized"]
                 else "unrecognized")
        print(f"{r['title_id']}  {label:<28} {r['path']}")
    return 0


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    if not argv or argv[0].startswith("-"):
        argv = ["build"] + argv      # bare flags -> build
    args = build_parser().parse_args(argv)
    args._repo_root = ROOT
    command = args.command or "build"

    if command == "roms":
        return cmd_roms(args)
    if command == "readme":
        return cmd_readme(args)
    if command == "summary":
        data = collect.collect(args, rom=resolve_rom(args))
        page.render_summary(data, stream=sys.stdout)
        return 0
    if command == "serve":
        rebuild(args)
        from . import server
        server.serve(args, args.out, args.port, watch=args.watch)
        return 0
    rebuild(args)
    return 0
