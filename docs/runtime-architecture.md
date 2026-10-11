# Runtime architecture — Pokémon X/Y native recomp project

Status: strategy document, 2026-10-11. Grounded in the current state of this
repo: 3dsrecomp port generated and linked (`xy-recomp/build.sh`), 134/134
modules byte-identical relinks in the matching pipeline, 38,385 decompiled C
functions in the Ghidra corpus, and the `xy-extraction` toolkit with
validated decoders for every major asset family. Progress numbers below come
from the live heatmap (`xy-heatmap-py/heatmap.py`).

## 0. Where we stand (measured, not estimated)

| Signal | Value | Source |
|---|---:|---|
| Function inventory (all tools, union) | 72,418 | port headers ∪ Ghidra CSVs ∪ decomp corpus ∪ export tables |
| Identified/named functions | 592 (0.8 %) | static.crs 461 exports + CRO export tables (179 unique) + stray Ghidra names |
| Decompiled to C | 38,385 (53 %) | `decomp/code_c` + `decomp/cro_c/<Dll>` |
| Recomp-covered (3dsrecomp) | 69,038 (95 %) | port `functions.h` / `m*_functions.h` |
| Natively verified (per-function) | 0 — harness ready | future: 3dsrecomp verify / objdiff per-unit |
| Module relinks byte-identical | 134/134 | `pipeline/orig` vs `pipeline/out` |

The bottleneck is **identification, not coverage or decompilation**: the
recompiler already covers 95 % of code and Ghidra produced C for half of
everything, but almost nothing carries a real name yet. The runtime tracks
below are gated on naming + verification work, which is exactly what the
heatmap GUI exists to steer.

## 1. Two-track strategy

**Track A — Zakuro fork (hybrid fallback runner).** The existing
`3dsrecomp port` flow produces a standalone binary: recompiled C linked
*into* Zakuro via `zakuro::Linked::new(title_id, recomp_tables…)` +
`zakuro::run()`, with **all assets still read from the .3ds** at runtime.
This boots today (build.sh, `--run`). We fork Zakuro and refine it — same
core structure (crates `zakuro-core/-cpu/-gpu/-fs`), but tuned to run *our*
recomp output as the primary path with the interpreter only as a safety net.

**Track B — full native custom runtime (primary goal).** A purpose-built
runtime that executes the recompiled code against our own HLE layer and
serves assets from the **extracted/curated tree** (`xy-extraction` output)
instead of the ROM container — no emulator shell, no .3ds dependency at
ship time.

Both tracks share everything upstream: the same 3dsrecomp lifting, the same
symbol/namespace identification, the same verification ladder. Track A is
not throwaway — it is the executable spec for Track B's service layer and
our oracle for differential testing.

## 2. 3dsrecomp mechanics (what the lifting actually gives us)

- **Function discovery**: entry point → calls → exports → relocations →
  data pointers → inter-module imports, then start detection on the
  remainder. Result for XY: ~69 k function starts across Main + 133 CROs
  (it merges some functions Ghidra splits and vice versa — hence the
  heatmap's union inventory).
- **Codegen**: ARM, Thumb and VFP → C with a `Context` carrying the
  register file; rare instructions fall through to the interpreter.
  Functions are `void f_XXXXXXXX(Context*)` in per-module headers
  (`functions.h` absolute 0x00100000-based, `m<NNN>_functions.h`
  module-relative 0x180-based).
- **CRO dynamic modules**: "modules get code that works wherever the game
  loads them" — the port emits position-independent module code plus
  `entries.c` tables (name, base, size, declared count). At runtime the
  module base floats; this is what makes XY's 133-CRO DLL graph statically
  recompilable at all.
- **Overrides**: any function can be replaced by handwritten C
  (`xy-recomp/overrides/`) — for fixes, mods, or fast paths. This is the
  seam where decompiled-and-verified source replaces raw lifted code
  incrementally.
- **Verify mode**: runs every recompiled function against Zakuro's
  interpreter from the same state and compares registers/flags/memory —
  the future feed for the heatmap's *Natively Verified* tier.
- **Hints loop**: Zakuro records where it still had to interpret; the next
  build takes those addresses in (`--hints` on the heatmap, hints file on
  the recomp build). Coverage ratchets up per build.

## 3. Track A — Zakuro fork, refined

What exists today (`xy-recomp/build.sh`):

```
.3ds → 3dsrecomp port --zakuro .thirdparty/zakuro
     → .Data/Curated/ports/pokemon-x/   (cargo project)
     → cargo build --release            (ccache-accelerated)
     → target/release/pokemon-x <rom>   (standalone binary)
```

The fork refines, in order:

1. **Repath the runtime dependency** to our fork (the generated port's
   `Cargo.toml` points at a stale pre-rename path — regenerate with
   build.sh, which already resolves `.thirdparty/zakuro`).
2. **Drive the hints loop in CI**: run → collect interpreter hits →
   rebuild. Zakuro's own numbers for Pokémon Y: ~99.8 % of executed
   instructions come from the recompiled library; our goal is to push the
   residual toward zero and watch the *interpreted* tier in the heatmap
   shrink.
3. **Swap the asset source** behind Zakuro's FS layer: RomFS reads can be
   redirected from the .3ds to a directory (its mod `romfs/` override
   machinery already does per-file replacement — generalize it to mount a
   full extracted tree). This is the bridge experiment for Track B's asset
   pipeline, validated inside a known-good runner.
4. **Headless/deterministic mode** for automated differential testing
   (fixed RNG, scripted input, frame-hash comparison) — the oracle both
   tracks use.
5. Keep its mod/cheat/texture-pack compatibility surface intact (Luma/
   Azahar-compatible layout) — free ecosystem support.

Exit criteria for Track A as *fallback*: full playthrough of X on the
fork with recomp coverage ≥ 99.9 % of executed instructions and assets
loadable from both .3ds and extracted tree.

## 4. Track B — full native custom runtime

### 4.1 Layer map

```
┌────────────────────────────────────────────────────────────────────┐
│ Pokémon X recompiled code (3dsrecomp output + verified overrides)  │
│   Main (code.bin @0x00100000) + 133 CRO modules (PIC)              │
├────────────────────────────────────────────────────────────────────┤
│ Runtime core                                                       │
│   • ARM11 core state (Context) + interpreter fallback for residue  │
│   • CRO loader: load bases, relocation application, import/export  │
│     resolution against static.crs tables (semantics from our       │
│     validated xy-archive/cro.rs parser — reuse as the reference)   │
│   • OS emulation: threads/sync (SVC), timers, exceptions           │
├────────────────────────────────────────────────────────────────────┤
│ Service HLE (Azahar is the reference implementation)               │
│   FS · APT (app lifecycle) · GSP (GPU commands, framebuffer) ·     │
│   DSP (audio) · HID (input) · AM/CEC/HTTPC/AC … as the game calls  │
├────────────────────────────────────────────────────────────────────┤
│ Asset bridge (this repo's expertise — see 4.2)                     │
│   curated tree / RomFS mount → decoded resources → game-visible    │
│   buffers (or fed to render/audio backends directly)               │
├────────────────────────────────────────────────────────────────────┤
│ Host backends: GPU (PICA command lists → host renderer), audio     │
│   mixer, input, filesystem                                         │
└────────────────────────────────────────────────────────────────────┘
```

The critical insight from Azahar study: the game never touches hardware
directly — it builds **PICA200 command lists** and hands them to GSP, and
reads assets through **FS service calls with game-side archive mounting**
(GARC "archives" are opened via FS and parsed by game code). So Track B
doesn't need a GPU emulator; it needs (a) correct service semantics and
(b) a command-list processor that turns PICA state (shaders, texture
formats, geometry) into host draws. Azahar's source (in
`.thirdparty/reference/`) is the primary study for both, exactly as its
SMDH decoder already became our icon goldens.

### 4.2 The asset bridge — how logic meets data

Two mount modes, same decode layer:

- **ROM mode**: mount RomFS from the .3ds directly (Track A parity; the
  game's own code parses GARCs etc. itself — we only provide bytes).
- **Native mode**: serve the **curated tree** (`.Data/Source_Extracted/`
  and `xy curate` output). The runtime's FS HLE translates the game's
  archive/file opens onto the extracted layout, and validated `xy` decoders
  become the runtime's resource loaders.

Per-family plan (decoder status = validation state in `xy-extraction`):

| Family | Location | Decoder (reference port) | Runtime representation | Status |
|---|---|---|---|---|
| Pokémon models | `a/0/0/7` PC/BCH | `xy-gfx::bch` (SPICA) | mesh+skin+material+anim | ✅ 981/981 |
| Textures | PC/PT/CGFX | `xy-gfx::texture` (Ohana3DS) | decoded RGBA | ✅ all 14 PICA fmts |
| World/effect models | CGFX rev-9 | partial — CNOD chunk | — | ⛔ open RE (CNOD) |
| Motions | `a/0/9/{3,4}` GFMotion v2 | none public | — | ⛔ open RE (v2 bitstream) |
| Per-form anims | PF BCH | `xy-gfx::bch` anims | curve data | ✅ (material/vis) |
| Music | `sound/*.aac` | ffmpeg | host decode | ✅ (std ADTS) |
| SFX | BCWAV | `xy-audio::bcwav` | DSP-ADPCM → PCM | ✅ byte-parity |
| Video | `m/*.moflex` | ffmpeg | host decode | ✅ |
| Text banks | `a/0/7`, `a/0/8` | `xy-script::textfile` | JSON | ✅ 4,600/4,600 |
| GARC archives | everywhere | `xy-archive::garc` | virtual FS | ✅ 272/272 |
| CRO modules | `cro/` | `xy-archive::cro` | load/link | ✅ 134/134 |
| Shaders | in-model PICA cmd | — | translate to host | ⛔ Track B work |
| Banner/icon/smdh | ExeFS | `xy-gfx::smdh` | — | ✅ Azahar-parity |

Implication: **the runtime's GPU work is the true hard part of Track B** —
geometry, textures, and skeletons are already solved by our own toolkit,
and audio/text/video are done. The two open RE projects (CNOD geometry,
GFMotion v2) gate *fidelity* (world models, full skeletal animation), not
architecture; until cracked, those families fall back to ROM mode.

### 4.3 Role of the C# translation layer

`curated/code/csharp/` (mechanical C→C# of the whole corpus) is **not**
runtime code. Its value:

1. **Readable second view** of the 38 k-function corpus — cross-check a
   suspicious Ghidra decompilation against an independently-transformed
   rendering of the same body.
2. **Algorithm prototyping**: validate understanding of a game system
   (save format math, RNG, battle resolution) in a memory-safe harness
   before committing a handwritten C override into the recomp.
3. **Tooling IR** for future automated diffing (decomp vs recomp vs C#).

### 4.4 Verification ladder (shared by both tracks)

```
raw  →  identified  →  decompiled  →  matching  →  natively verified
name      symbol table     Ghidra C      objdiff       3dsrecomp verify
sources:  static.crs       corpus        per-unit      (registers/flags/
          CRO exports      (done: 53%)   build (harness   memory equal)
          RTTI lever                      proven 134/134
          heatmap: green                  module-level)
```

Each step feeds the heatmap's tiers; the GUI is the steering instrument
for this ladder, not decoration.

## 5. Track A vs Track B — structured comparison

| Dimension | A: Zakuro fork | B: full native runtime |
|---|---|---|
| Game code | recomp linked in (`zakuro::Linked`) | recomp linked in (own core) |
| Residual code | Zakuro interpreter | own ARM11 interpreter fallback |
| Assets | .3ds (→ extracted-tree mount later) | curated tree first, .3ds optional |
| GPU | Zakuro GPU (PICA → host) | own command-list processor (Azahar-informed) |
| Audio | Zakuro DSP HLE | own mixer; BCWAV/AAC decoders ported from xy-audio refs |
| Dynamic CROs | Zakuro loader | own loader (cro.rs semantics) |
| Maturity | boots/played today (XY: Playable upstream) | greenfield, phased |
| Mod support | Luma/Azahar-compatible, free | must reimplement surface |
| Determinism | to add | designed-in (testing oracle) |
| Risk | low — contained changes | high — service/GPU semantics |
| End state | fallback runner + oracle | shippable native game |

Sequencing: A now (fallback + differential oracle), B grows underneath it
(core → services → GPU) with A as reference at every step.

## 6. Milestones

- **M0 (done)**: extraction toolkit validated; matching pipeline relinks
  134/134; recomp port builds+runs; heatmap GUI live.
- **M1 — Identification push**: RTTI lever + CRO export propagation into
  Ghidra CSVs; named functions 592 → ≥ 5 k (heatmap green density).
- **M2 — Zakuro fork v1**: repathed fork in-tree, hints loop in CI,
  headless deterministic mode; interpreted tier measurable in heatmap.
- **M3 — Asset bridge**: curated-tree mount inside Track A (FS redirect);
  xy decoders callable from the runtime as resource loaders.
- **M4 — Track B core**: own Context/interpreter + CRO loader + FS/APT/HID
  HLE, booting recomp Main headlessly against extracted assets.
- **M5 — Track B GPU/audio**: PICA command processor + mixer; first
  in-game frame rendered natively; per-function verify tier feeding the
  heatmap gold.

## 7. The live heatmap GUI (progress instrumentation)

Architecture:

```
.Data/Source_Extracted/
  recomp/{entries.c,*_functions.h}     ┐
  pipeline/symbols/*.csv               │
  cro_extracted/{*,static}/symbols.tsv ├─ xy-heatmap-py/heatmap.py ───→ heatmap.html
  decomp/{code_c,cro_c}                │   (self-contained, data      heatmap_data.json
  pipeline/{orig,out} (link check)     ┘    embedded; zero deps)      summary.md
                                                                     ↑ polled every 3 s
                                            browser (single page) ────┘
                                            canvas grids · tooltips · namespace
                                            filter · import graph · PNG/SVG export
```

- Run: `python3 xy-heatmap-py/heatmap.py serve --watch --port 8765`
  (or `build` / `summary` / `roms [--rom <file>]` — the CLI identifies
  .3ds/.cxi dumps by title ID and labels the dashboard accordingly);
  or open `heatmap.html` directly from disk.
- **Central radial canvas**: one disc, every module a sector (angle ∝
  function count, alphabetical from 12 o'clock), cells filling outward in
  address order — the whole corpus at a glance. Click a sector to jump to
  its module grid; radial SVG/PNG export included.
- Cell = one function (union inventory); colour = highest stage:
  dark grey raw → teal decompiled → green identified → gold verified;
  red overlay = interpreter hints. Recomp-coverage is a per-function bit
  surfaced in tooltips and per-module/global stats.
- Exports: per-module and whole-corpus SVG (vector, arbitrary zoom) and
  PNG (rasterized at 2–3×); every SVG rect carries a `<title>` with
  address + symbol.
- Extension points already wired: `--hints <file>` (Zakuro interpreter
  hits) and `--verified <json>` (`[{module, addr}, …]` from a future
  3dsrecomp-verify / objdiff export) — tier 3/4 light up when those feeds
  exist, no GUI change needed.

## 8. Risks / open items

- **PICA200 shader semantics** — the largest Track B unknown; Azahar +
  homebrew shader docs; game shaders live inside models (cmd buffers).
- **CNOD geometry & GFMotion v2** — open RE projects (see PLAN.md
  references); gate world-model fidelity, not architecture.
- **Identification ceiling** — without symbols, overrides and verification
  targets are guesses; M1 is the multiplier for everything downstream.
- **v1.5 update partition** — port currently built from base v0; update
  code.bin (5,169,152 B) must become either a second port variant or the
  canonical basis before ship.
