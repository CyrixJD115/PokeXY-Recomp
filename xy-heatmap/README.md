# xy-heatmap — progress-heatmap studio

One self-contained package for the Pokémon X/Y recomp + decomp effort: it
turns the corpus (Main + 133 CRO modules, one cell per function in the
union inventory) into an **interactive local web UI** and **deterministic
SVG/PNG exports** — rendered from one shared scene, writing nothing to
disk unless you ask for an export.

**Rust (axum) backend + Svelte 5 / TypeScript frontend** — the Python
implementation it replaces produced byte-identical SVG output (the port is
parity-tested against it); see `src/` for the single Rust crate and `ui/`
for the SPA.

## Quick start

```bash
# UI dev loop (two terminals): Rust server on 8765 + Vite dev server with proxy
cargo run -- serve --watch --port 8765      # from xy-heatmap/ — API + views
pnpm --dir ui dev                           # open the printed Vite URL (proxies /api,/view)

# production: build the SPA, then one static binary serves everything
pnpm --dir ui build
cargo build --release                        # ui/dist is embedded (rust-embed)
./target/release/xy-heatmap serve --watch --port 8765

# headless commands (no UI needed)
cargo run -- summary                                        # text table to stdout
cargo run -- export --view treemap --format png --out t.png --scale 3
cargo run -- export --view module:DllBag --format svg --out bag.svg
cargo run -- export --view all --format svg --out assets/   # all four views
cargo run -- readme                                          # regen docs/assets/heatmap-*.svg + progress-bar.png

cargo test                                                   # test suite (see selftest below)
```

Works from any working directory if you pass `--data-root` (or set
`XY_HEATMAP_DATA_ROOT`). **`serve` writes nothing, ever** — the dataset
payload, scene JSON and every rendered view are served from memory; Ctrl-C
stops cleanly.

## Views

| View | What it shows |
|---|---|
| `treemap` | **hybrid**: big modules share a squarified treemap (area = function count, strong grid-line borders, per-function cells); small modules (≤ `--minor-max`, default 200 fns) become fixed-size cards below that fit **all** their cells — no slivers, no truncation |
| `progress` | the **overall progress banner**: one 100%-stacked bar of the whole corpus by highest stage reached (raw → decompiled → named → verified, in-segment percentages) plus a recomp-covered progress line — exported as `heatmap-progress-bar.png`, embedded in the root README |
| `overview` | one 100%-stacked tier-composition bar per module, sorted by function count (this is `heatmap-progress.svg`) |
| `grid` | the **whole corpus as one address-ordered grid**, grouped into labeled per-module sections — a first-class alternative to the bars |
| `namespaces` | subsystems visible in named symbols (demangled C++ paths, Ghidra labels): each row shows the count, the share of the corpus, and **which modules** those symbols live in; the unnamed remainder is called out explicitly |
| `module:<NAME>` | one module's functions, address-ordered, with a facts header (counts, coverage, relink, imports) |

Status ladder (color = highest stage reached): **raw** (grey) →
**decompiled** (teal) → **named** (green) → **verified** (gold); red ring =
interpreter hint. Recomp-coverage is a per-cell flag surfaced in tooltips
and stats, never a color step.

All views accept a `min-tier` filter (exports: `--min-tier`; server: `?min=`;
UI: the tier dropdown) and a column count for grids (`--cols` / `?cols=` /
the density dropdown).

### The web UI is a fixed dashboard — no canvas

The page is a structured layout: header (global stats) → toolbar (view tabs,
tier filter, density, SVG/PNG export) → **sidebar with every module** (tier
mini-bar, function count, relink mark, filter box) → main panel → **readout
bar** (point at any cell: module, address, tier, symbol) → legend footer.
There is no panning, dragging or zooming anywhere — every view is an SVG in
normal document flow scaled to the available width, and the page scrolls
like any web page. Clicking a module anywhere (treemap box, sidebar row,
bar row) opens its **detail page**: facts card (counts, coverage, address
range, relink status, **imports from** — the real dependency data from
`imports.tsv`), then the address grid. Views deep-link via the URL hash
(`#grid`, `#module-DllField`, …) and `--watch` live-reloads when the
sources change (client polls `/api/fingerprint` every 3 s).

The frontend never computes layout: the server exposes the scene as JSON
(`/api/view/<spec>`) and `SceneView.svelte` renders it; the SVG/PNG
downloads come from the same layout via the `/view/<spec>.svg|.png`
endpoints — interactive view and exports are identical by construction.

## Where the data comes from

Read-only collection from a `Source_Extracted` root (all frames
canonicalized: Main = absolute 0x100000, CROs = module-relative 0x180):

| Source | Gives |
|---|---|
| `recomp/entries.c` + `*_functions.h` | module table (name ↔ m-number) + recomp-covered starts |
| `pipeline/symbols/*.csv` | Ghidra starts, sizes, real names, namespaces |
| `cro_extracted/**/symbols.tsv` | real export symbols (Thumb bit stripped for `static`) |
| `decomp/code_c`, `decomp/cro_c/<Mod>` | decompiled-to-C coverage |
| `pipeline/orig` vs `pipeline/out` | per-module SHA-256 relink check |

Any source may be missing — the collector warns and continues; only a
malformed `entries.c` is fatal.

### Standalone datasets

Without any project data, feed a generic JSON file (`--dataset file.json`):

```json
{
  "title": "my corpus",
  "modules": [
    {"name": "DllBag", "relink_ok": true,
     "cells": [
       {"addr": "0x180", "size": 64, "name": "Bag_Open",
        "covered": true, "decompiled": true, "named": true, "hint": false}
     ]}
  ]
}
```

`addr` is an int or decimal/`0x`-hex string; tiers escalate
(`raw < decompiled < named < verified`); every field except `addr` is
optional. Invalid input is rejected with a precise message.

## Architecture

```
src/dataset.rs    data model + validation + generic JSON input (serde)
src/collect.rs    read-only project collectors + content fingerprint (live reload)
src/config.rs     every presentation knob in one place (shared by UI and exports)
src/palette.rs    the status-tier color ladder
src/scene.rs      resolution-independent primitives (Rect/Text/Line/Cells) + serde
src/layout.rs     pure geometry: hybrid treemap, corpus/module grids,
                  composition bars, namespace breakdown
src/svg.rs        Scene -> SVG (hand-written serializer; byte-identical output;
                  Cells render as one path per tier)
src/png.rs        Scene -> PNG (resvg rasterizer; system fonts via fontdb,
                  XYHEATMAP_FONT override; pHYs DPI metadata)
src/views.rs      view-name -> Scene, shared by CLI and server
src/server.rs     axum: /api/dataset, /api/fingerprint, /api/module/<name>,
                  /api/view/<spec> (scene JSON), /view/<spec>.svg|.png,
                  embedded SPA (rust-embed of ui/dist)
src/cli.rs        serve | export | summary | readme
ui/src/App.svelte the fixed-dashboard shell (tabs, sidebar, facts, readout,
                  watch polling, hash routing)
ui/src/SceneView.svelte  renders scene JSON as SVG + hover decode (data-cells)
```

Tests: `cargo test` — collectors against a synthetic `Source_Extracted`
tree (never the real `.Data`), dataset validation, squarify invariants,
SVG/PNG byte-determinism, router routes. `summary` and the committed
`docs/assets/heatmap-*.svg` were parity-checked byte-for-byte against the
Python implementation this package replaced (2026-10-15).

`shell/rebuild_v2.sh` lives here for historical reasons — it is the
repo-wide `Source_Extracted` re-dump script, not part of the package.
