# xy-heatmap-py — progress heatmap toolkit

One CLI, one dashboard for the Pokémon X/Y recomp + decomp effort:
a **central radial heatmap** (every module a sector of a single canvas),
per-module address-ordered grids, namespace and import-graph panels,
PNG/SVG export, and a live mode that re-renders as the data changes.

## Quick start

```bash
python3 xy-heatmap-py/heatmap.py                  # build the dashboard
python3 xy-heatmap-py/heatmap.py serve --watch    # live: rebuild + serve
# then open http://127.0.0.1:8765/heatmap.html

python3 xy-heatmap-py/heatmap.py summary          # text table to stdout
python3 xy-heatmap-py/heatmap.py roms             # list .Data/Rom dumps
python3 xy-heatmap-py/heatmap.py roms --rom path/to/Y.3ds
```

Works from any working directory (defaults anchor to the repo root), and
`uv run heatmap.py …` works from inside this directory.

## What it reads (all under `.Data/Source_Extracted/` unless overridden)

| Source | Gives us |
|---|---|
| `recomp/entries.c` + `*_functions.h` | module map + 3dsrecomp function starts (recomp-covered bit) |
| `pipeline/symbols/*.csv` | Ghidra function starts + occasional real names |
| `cro_extracted/<Mod>/symbols.tsv` (+ `static/`) | the **real** export symbols (mangled C++) |
| `decomp/code_c` + `decomp/cro_c/<Mod>` | decompiled-to-C corpus coverage |
| `pipeline/orig` vs `pipeline/out` | per-module byte-identical relink check |

The function grid is the **union** of every tool's function starts — the
port merges some functions Ghidra splits and vice versa, so no single
source is authoritative.

## Status ladder (cell colour)

| Tier | Meaning | Feed |
|---|---|---|
| Raw Binary (dark grey) | function start known, nothing else | union inventory |
| Decompiled to C (teal) | Ghidra C exists, still unnamed | decomp corpus |
| Identified / Named (green) | carries a real symbol | export tables + CSVs |
| Natively Verified (gold) | recomp verified against original | `--verified` JSON (future objdiff/3dsrecomp-verify export) |
| Interpreted overlay (red) | Zakuro interpreter hit | `--hints` file |

Recomp-covered (3dsrecomp port start) is tracked per function and shown
in tooltips and module/global stats — not a colour.

## Central radial canvas

Sector angle ∝ function count, modules alphabetical from 12 o'clock
clockwise; cells fill each sector from the center outward in address
order (inner rings = lowest addresses). Hover for details, click a sector
to jump to that module's grid. Export as SVG (vector, tooltip titles on
every wedge) or PNG (3× raster) — same for per-module grids and the
whole-corpus strip via the header buttons.

## Layout

```
heatmap.py          CLI entry (thin shim)
xhmap/              the package
  cli.py            subcommands + flags
  romdetect.py      .3ds/.cxi title-ID identification (--rom)
  collect.py        data collection (sources, tiers, namespaces)
  page.py           dashboard template + output writers
  server.py         --watch fingerprint loop + --serve http
shell/
  rebuild_v2.sh     full clean-slate re-dump of .Data/Source_Extracted
legacy/
  heatmap_v1.py     the original per-module SVG generator (superseded)
  legacy-build-pokemon-x-port.sh   pre-build.sh port shim (superseded
                                   by xy-recomp/build.sh)
```

Outputs land in `.Data/Source_Extracted/heatmap_gui/`: `heatmap.html`
(self-contained, works from file://), `heatmap_data.json` (live-poll
payload) and `summary.md`.
