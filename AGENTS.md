# AGENTS.md

Agent guidance for **PokeXYZ-RDComp** — the combined Rust toolkit for the
Pokémon X/Y native-recomp + decomp project: format extraction & conversion
(`xy-extraction/` — the `xy` CLI), static recompilation (`xy-recomp/`:
3dsrecomp + Zakuro under `.thirdparty/`), matching decompilation
(`xy-decomp/`: 3DS-Decomp-Pipeline + objdiff + Ghidra automation), and the
native runtime track (`xy-runtime/`, zakuro-based). Python appears only as
helper scripts under `xy-heatmap-py/`; Rust is the daily driver.

## Repo layout

| Path | Contents |
|---|---|
| `xy-extraction/` + `src/` | the extraction toolkit — 8 domain crates (`xy-core/archive/audio/gfx/script/battle/curate/convert`) behind the unified `xy` CLI: unpack → curate → convert |
| `xy-recomp/` | recomp track: `build.sh` port builder, C `overrides/` |
| `xy-decomp/` | matching-decomp track: `ghidra/` (patched 3ds suite + PyGhidra driver + symbol exporter), track README |
| `xy-runtime/` | runtime track: zakuro-based native runtime (working clone gitignored, README tracked) |
| `xy-heatmap-py/` | the progress-heatmap toolkit: `heatmap.py` CLI (build/serve/summary/roms) over the `xhmap/` package (**central radial heatmap** + per-module grids, PNG/SVG export, ROM detection with `--rom`, live `serve --watch`; see `docs/runtime/runtime-architecture.md` §7) + `shell/rebuild_v2.sh` + `legacy/` (superseded v1 scripts) — README inside |
| `.thirdparty/` | machine-local upstream checkouts (3dsrecomp, zakuro, 3DS-Decomp-Pipeline, objdiff) — `fetch.sh` recreates them |
| `.Data/` | **machine-local, never share**: `Rom/` (dumps + keys + v1.5 update partitions), `Raw_Extracted/` (read-only sources), `Source_Extracted/` (all derived output — index: `.Data/Source_Extracted/MAP.md`), `Curated/` (final outputs: `ports/`) |
| `docs/` | **the research log — every finding lands here** (see Documentation duty below; `docs/README.md` is the map + "what to explore" frontier list): topic folders `3ds/ formats/ knowledge/ dll/ decomp/ recomp/ runtime/ general/` — incl. `general/findings.md` (measured run metrics), `decomp/ghidra-scripts.md` (PyGhidra setup + patches), `runtime/runtime-architecture.md` (two-track strategy), `general/PokemonX_Decomport_Report.md` (founding survey) |
| `reference/` | early python prototypes kept for parity reference |

**Directory convention:** dot-prefixed top-level dirs (`.Data/`,
`.thirdparty/`) are machine-local workspace material ("agentic
references") — never committed except their README/index files; non-dot
dirs are repo content. Game tracks are `xy-`-prefixed (`xy-extraction/`,
`xy-recomp/`, `xy-decomp/`, `xy-runtime/`); the extraction toolkit's CLI
package (`src/` + root `Cargo.toml`) lives at the repo root.

### Key data landmarks (under `.Data/`)

| Path | Contents |
|---|---|
| `.Data/Rom/X/Base/`, `.Data/Rom/Y/Base/` | Source cartridge dumps — title ID `...0055D00` = **Pokémon X** (internal name `kujira-1`, confirmed by its own SMDH long name in all 12 regions), `...0055E00` = **Y** (`kujira-2`); each with its decrypted `.dec.code` |
| `.Data/Rom/X/Upd/`, `.Data/Rom/Y/Upd/` | **v1.5 update partitions** (title `0004000E…` CXI + decrypted v1.5 `code.bin`, 5,169,152 B) |
| `.Data/Rom/nand/` | Console key material (`boot9.bin`, `firm0.bin`, `otp.mem`) — never commit or share |
| `.Data/Raw_Extracted/` | Raw decrypted RomFS/ExeFS/partition dumps — treat as **read-only source data** |
| `.Data/Source_Extracted/garc/` | All 272 GARCs unpacked (44,519 files) + `manifest.tsv` |
| `.Data/Source_Extracted/decomp/` | `code.bin` + Ghidra project + 38,385 decompiled C functions (14,864 code.bin + 23,521 across all 133 CRO modules) |
| `.Data/Source_Extracted/pipeline/` | Matching-decomp workspace: `orig/` + `symbols/` CSVs + generated `split/` + `out/` (134/134 byte-identical recreates) + `objdiff.json` |
| `.Data/Source_Extracted/recomp/` | 3dsrecomp analyze/build output + the standalone port project (`port/`) |
| `.Data/Source_Extracted/heatmap_gui/` | generated dashboard (`heatmap.html` + `heatmap_data.json` + `summary.md`) from `xy-heatmap-py/heatmap.py` |
| `.Data/Source_Extracted/text/` | gametext/storytext JSON, 8 languages (4,600 banks) |
| `.Data/Source_Extracted/cro/` | The 133 plaintext `.cro` modules + `static.crs` (CRO0 magic at 0x80 — parser landed; **not** encrypted) |
| `.Data/Curated/` | Final outputs — `Curated/ports/` holds the port trees `xy-recomp/build.sh` generates |
| `.thirdparty/reference/` | Third-party **reference** library (azahar source, CTR format tools, pk3DS/XAMX/Ohana3DS format references, Ghidra script suites incl. the RTTI lever) — gitignored, has its own README index; the four tools this repo executes directly are in `.thirdparty/` proper. The old `PokeXYZ-PC/` project space was deleted 2026-10-11 — this repo is the only home. |


Romfs quick map (Pokémon X/Y):

- `sound/*.aac` — BGM as plain ADTS AAC (48 kHz stereo). Decode with ffmpeg;
  not a custom format — don't write a parser for it.
- `m/*.moflex` — movies (Mobiclip video + IMA ADPCM audio). ffmpeg handles them.
- `a/`, extensionless files, `*.garc` — GARC archives (**parser landed**:
  `xy archive unpack -i <garc> -o <dir> --decompress`).
- `*.cro`, `static.crs`, `.crr/` — **CRO code modules: plaintext**
  (parser landed: `xy cro info/extract`). Magic `CRO0` at **0x80** — the
  first 0x80 bytes are SHA-256 hashes and look encrypted; they aren't.
- `shaders/` — 4 SDK system `.shbin` shader binaries (game shaders live
  inside the BCRES model files).
- `part7_romfs` (in `Raw_Extracted/`) — card update partition: 64
  system-title CIAs, zero game content; not processed.
- Completeness audit 2026-10-05 (`xy archive audit`): 14,452 0x11-prefixed
  sub-files, 14,409 decompress, the 43 raw keeps are genuinely raw —
  **no missed compressions**; full format census in MAP.md.
- `sounddebug/` — plain text debug output, not audio.
- `Source_Extracted/banner_jingle.bcwav` — banner sound, the first format this
  toolkit parsed.

## Documentation duty — every finding lands in `docs/`

This project's real output is knowledge; code is how we verify it. **A finding
that isn't written down did not happen.** `docs/README.md` is the map — read it
before exploring, extend it when you learn something; it also carries the
"what to explore" frontier list (the open RE problems worth fighting).

- **Always log what you find, same session.** Any durable discovery — a format
  quirk, a working command/flag set, a tool gotcha, a mapping rule, a
  measurement, a dead end, a disproven theory — goes into the matching
  `docs/<folder>/*.md` file before you report the task done. Dead ends count
  double: record what was tried and why it's wrong, marked
  "wrong, kept for history" (this file already does).
- **Reference, don't duplicate.** If a doc already covers the topic, append a
  dated entry to it instead of creating a near-duplicate; link `path:line`,
  the parser module doc, or the upstream tool rather than restating it.
- **AGENTS.md is the condensed layer.** It keeps only what an agent needs to
  act on (conventions, commands, pointers). Detail lives in `docs/`; when a
  finding changes daily-driver behavior, update the one-liner here and link
  the doc that holds the detail.
- **Binary layout specs live in the parser module docs** (offset tables kept
  in sync with the code). `docs/formats/` holds the investigation narrative
  around them — recon notes, cross-tool parity, open problems — linked both
  ways when both exist.
- **Keep the map true.** Adding or repurposing a docs file or folder means
  updating `docs/README.md` (and that folder's README index) in the same
  change. `general/findings.md` stays the run-metrics log for
  recomp/decomp builds; new topical notes go to the folders.

## Goals & scope

- Target **Pokémon X first**, then Y — but formats are shared, so parsers stay
  game-agnostic. Do not hardcode title IDs in library code.
- Cargo **workspace**, eight domain crates + a root CLI package (subcommand
  groups map 1:1 onto crates):
  - `xy-extraction/xy-core` — shared primitives (typed errors, bounds-checked reads,
    LZ10/LZ11, `sniff()` magic identification, `json_escape`);
  - `xy-extraction/xy-archive` — containers (RomFS, BCMA/DARC, GARC, CRO);
  - `xy-extraction/xy-audio` — sound (BCWAV + WAV writer);
  - `xy-extraction/xy-gfx` — graphics (SMDH, PICA textures, CGFX, BCH, PC/PF/PT,
    OBJ export, **GLB/glTF 2.0 export**);
  - `xy-extraction/xy-script`, `xy-extraction/xy-battle` — text banks landed / battle stub;
  - `xy-extraction/xy-curate` — the curated-tree pipeline as a **library**
    (extracted 2026-10-06 from `src/curate.rs` so the CLI and any future
    front-end share one implementation; `build_with_progress` reports
    progress + honors a cancellation flag, `manifest` reads trees back);
  - `xy-extraction/xy-convert` — viewer/inspector/convert backend: kind detection
    (`detect_kind` → `FileKind`; runs on full buffers for GF containers,
    whose entry tables validate against the file length), whole-model
    intermediate (`model::SceneModel` from BCH/CGFX/containers), one-shot
    conversions (GLB/PNG/WAV/JSON as in-memory `OutputFile`s), inspection
    +hashing (`inspect`), ffmpeg/ffprobe orchestration (`media`), headless
    software 3D rasterizer (`render`: textured|material|wireframe|normals|
    skeleton, turntable, `--textures` pack merge). Validated 2026-10-06 by
    rendering real `pm0001_00` (Bulbasaur: geometry, UVs and family
    textures all bind correctly — green skin/bulb/red eyes) in ~5 ms.
  - root package `xy` (`src/main.rs`) is the unified CLI binary.
  Keep library dependencies minimal and logic-only: `thiserror` for typed
  errors, `png` only inside `xy-gfx` (whose error wraps core + png); the
  binary adds `clap`. No I/O inside parsers — they borrow input slices and
  return data. **The front-end is headless CLI only** (2026-10-06: a GUI
  package was prototyped and removed the same day by decision — do not
  reintroduce UI dependencies into library crates).

## Commands

```bash
cargo build            # dev build
cargo build --release  # optimized (strip + LTO)
cargo test             # unit + integration (integration skips if ROM assets absent)
cargo clippy --all-targets -- -D warnings
cargo fmt              # rustfmt defaults

# progress dashboard (central radial heatmap + module grids; runs from any cwd)
python3 xy-heatmap-py/heatmap.py                        # build (one-shot)
python3 xy-heatmap-py/heatmap.py serve --watch --port 8765   # live page + rebuild on change
python3 xy-heatmap-py/heatmap.py summary                # text table to stdout
python3 xy-heatmap-py/heatmap.py roms [--rom <file>]    # identify .3ds/.cxi dumps (X/Y × base/upd)

# examples (run from repo root; binary is `xy`)
./target/release/xy info  .Data/Rom/X/Base/0004000000055D00_v00.3ds
./target/release/xy audio bcwav .Data/Source_Extracted/banner_jingle.bcwav -o out.wav
./target/release/xy gfx icon   .Data/Raw_Extracted/exefs/icon.bin -o icon.png
./target/release/xy archive romfs .Data/Rom/X/Base/0004000000055D00_v00.3ds out_dir/  # CCI -> full RomFS
./target/release/xy archive darc  .Data/Raw_Extracted/part1_romfs/Manual.bcma out_dir/
./target/release/xy archive manual .Data/Raw_Extracted/part1_romfs/Manual.bcma out_dir/
./target/release/xy core lz file.lz -o file.dec
./target/release/xy archive list .Data/Raw_Extracted/romfs/btl_ai.garc
./target/release/xy archive unpack -i .Data/Raw_Extracted/romfs/a/0/1/2 -o out_dir/ --decompress
./target/release/xy archive extract-tree -i .Data/Raw_Extracted/romfs/a -o .Data/Source_Extracted/garc --decompress
./target/release/xy script text .Data/Source_Extracted/garc/a/0/7/4/080.bin -o names.json
./target/release/xy gfx bcres-info .Data/Source_Extracted/garc/a/0/0/1/0.bin
./target/release/xy gfx parse-model .Data/Source_Extracted/garc/a/0/0/7/0004.bin -o out_dir/  # BCH -> OBJ+MTL
./target/release/xy gfx texture     .Data/Source_Extracted/garc/a/0/0/7/0006.bin -o out_dir/  # BCH/CGFX/PC/PT -> PNG
./target/release/xy cro info .Data/Source_Extracted/cro/DllBattle.cro
./target/release/xy cro extract .Data/Source_Extracted/cro/DllBattle.cro -o .Data/Source_Extracted/cro_extracted/DllBattle
./target/release/xy archive audit --input .Data/Raw_Extracted/romfs/a  # compression audit
./target/release/xy curate -i .Data/Source_Extracted                     # full curated rebuild (all stages)
./target/release/xy curate -i .Data/Source_Extracted --stage pokemon,csharp  # subset: data,text,pokemon,media,code,csharp,unknown,all
./target/release/xy convert glb .Data/Source_Extracted/garc/a/0/0/7/0004.bin -o out_dir/ --textures .../0006.bin --validate
                         # BCH/CGFX/PC -> Blender-ready GLB: TRS skeleton, embedded
                         # PNG textures (--textures merges a family PT/CGFX pack),
                         # --validate parses every output with the gltf crate
./target/release/xy convert png .Data/Source_Extracted/garc/a/0/0/7/0006.bin -o out_dir/  # textures -> PNG
./target/release/xy gfx render .Data/Source_Extracted/garc/a/0/0/7/0004.bin --mode textured --yaw 35 --width 800   # software-render model -> PNG
./target/release/xy gfx render ...0004.bin --turntable 8 --out out_dir/  # 8-frame turntable sequence
./target/release/xy gfx render ...0004.bin --textures ...0006.bin --mode textured  # merge a family PT/CGFX texture pack
                         # (model PCs carry no textures; modes: textured|material|wireframe|normals|skeleton)
./target/release/xy inspect .Data/Source_Extracted/garc/a/0/0/7/0004.bin --hashes  # kind + magic + CRC32/FNV/SHA-256 + model/texture census
./target/release/xy search name .Data/Source_Extracted/garc 0004 --limit 50        # file-name substring search
./target/release/xy search kind .Data/Source_Extracted/curated model               # files whose detected kind matches (model/texture/...)
./target/release/xy media info .Data/Raw_Extracted/romfs/m/OP001.moflex         # ffprobe container/stream metadata
./target/release/xy media frames .Data/Raw_Extracted/romfs/m/OP001.moflex -o frames/ --fps 5 --max 100 --width 640
./target/release/xy media mp4 .Data/Raw_Extracted/romfs/m/OP001.moflex -o OP001.mp4  # moflex -> H.264+AAC MP4
./target/release/xy manifest list .Data/Source_Extracted/curated moves --limit 100 # browse manifest.tsv (filter + limit)
./target/release/xy manifest lookup .Data/Source_Extracted/curated <file>          # provenance: curated <-> raw source
./target/release/xy manifest validate .Data/Source_Extracted/curated               # re-verify rows on disk (existence + size)
./target/release/xy hex <file> --offset 0x100 --len 64                            # hex+ASCII dump of a byte window
```

## Curated extraction — the final extract → rebuild → convert → organize layer

`xy curate` builds `.Data/Source_Extracted/curated/`: a fully self-contained tree
of **real files (never symlinks)** with proprietary formats converted into
standard ones wherever a validated parser exists. Deterministic: sorted
iteration, stable names, no timestamps — same input, byte-identical
structure. Every file is recorded in `manifest.tsv` (source → curated path →
action → detail → bytes → status) and re-verified into `validation.tsv`
(existence, size, and magic checks: RIFF/ftyp/PNG/JSON/OBJ).

| stage | output | conversion |
|---|---|---|
| `data` | `battle/ world/ ui/ manual/ shaders/` | organized real copies of the pk3DS-mapped GARC data |
| `text` | `text/` | per-bank JSON copies + `text/organized/<lang>/` named banks (`species.json`=bank 080, `moves.json`=013, `abilities.json`=034, `items.json`=096, … per pk3DS `TextReference.GameText_XY`) + `index.json` |
| `pokemon` | `pokemon/` | all 981 `a/0/0/7` 8-file families copied under `families/pmNNNN_Species_FF/`; models → **OBJ+MTL** (`models/`, Blender-validated); textures → **PNG** (`textures/`); orphans → `unknown/` |
| `media` | `audio/ video/` | WAV / MP4 copies (ffmpeg conversions happen in `xy-heatmap-py/shell/rebuild_v2.sh`) |
| `code` | `code/` | CRO modules, extracted segments + symbols, the whole Ghidra C corpus, `modules.json` (segments/exports/import graph per module) |
| `csharp` | `code/csharp/<Module>/` | **C→C# deobfuscation pipeline** (see below) |
| `unknown` | `unknown/garc/<romfs path>/` | unmapped data preserved raw with READMEs — never discarded |

The output root gets a `README.md`; curate refuses to delete a directory it
did not write (guard: no `README.md` inside → error).

### C → C# pipeline (`src/curate/csharp.rs`)

Mechanical, deterministic conversion of the whole Ghidra corpus (code.bin =
module `Main` + all 133 CRO modules; ~38k functions, ~1.5 s). Key rules:

- Value integers widen to `long` (makes every int arg/return/assignment
  implicit); pointer types keep exact C widths (`int*` stays `int*`).
- `CONCATmn` → `Gh.CONCATmn` helpers; `&stack0x…` → `__stack + (off)` over
  `stackalloc`; `in_` pseudo-registers are plain declarations.
- Data labels become module fields: value labels `long` placeholders,
  pointer labels `byte*` pinned to their VA; string labels (`s_`/`u_`)
  pointer placeholders.
- Deref/indexed assignments get explicit pointee casts (`*(uint*)p = x` →
  `unchecked((uint)(x))` for literals); char literals → `(sbyte)`; C
  comma-expressions in `if`/`while` headers are hoisted/split.
- Functions using `code` pointers, indirect calls, or variadics are emitted
  as stubs with the original C commented; thunk signatures get `params`
  widening to match call sites; void thunks with consumed results widen to
  `long`; unknown externals get throwing stubs (`Imports.g.cs`); Ghidra
  synthetic helpers (`VectorUnsignedToFloat`, …) land in `Runtime.cs`.

Compile-checked with dotnet 10 per module: DllBattle 12 errors over 3,011
functions, DllBag 34 over 454 (most modules convert ≥60 % of functions;
stub-heavy ones less — per-module numbers in each README). The residue is
comma-expression edge cases and multi-line calls. The output is a readable
reconstruction — the decompiled C under `code/decomp/` stays authoritative.

## Architecture

```
Cargo.toml               # workspace root + root package `xy` (unified CLI)
src/main.rs              # clap CLI (xy): info | core | archive | audio | gfx
xy-extraction/xy-core/src/
  lib.rs                 # sniff() magic identification
  error.rs               # shared Error/Result (thiserror)
  util.rs                # bounds-checked *_le reads, UTF-16LE decode
  lz10.rs                # Nintendo LZ10/LZ11 decompressor
xy-extraction/xy-archive/src/
  romfs.rs               # RomFS/NCCH/CCI extraction
  bcma.rs                # XY e-manual DARC variant (12-byte nodes) — see module docs
  garc.rs                # GARC v4 game archives (CRAG/OTAF/BTAF/BMIF)
  lzss.rs                # LZ-0x11 payload decoder (dsdecmp flavor, pk3DS port)
  cro.rs                 # CRO0 code modules (info/extract; plaintext, magic @0x80)
xy-extraction/xy-archive/tests/
  garc_golden.rs          # golden tests vs .Data/Raw_Extracted/romfs (skips if absent)
xy-extraction/xy-audio/src/
  bcwav.rs               # CWAV parser (both layout variants) + DSP-ADPCM decoder
  wav.rs                 # canonical RIFF/WAV writer
xy-extraction/xy-gfx/src/
  error.rs               # gfx error (wraps xy-core Error + png)
  smdh.rs                # home-menu icon + title metadata
  texture.rs             # PICA200 texture decoder (all 14 formats, Morton,
                         # ETC1/ETC1A4; Ohana3DS TextureCodec port) + PNG writer
  bcres.rs               # BCRES/CGFX inspection (header/chunks/names)
  cgfx.rs                # full CGFX decoder: textures/materials/skeletons/
                         # skeletal anims (Ohana3DS CGFX.cs port); XY rev-9
                         # CNOD-linked mesh variant skipped (geometry_partial)
  bch.rs                 # BCH (CtrH3D) decoder: container + relocation +
                         # models/meshes/materials/bones/textures (SPICA port)
  pkmn.rs                # GF PC/PF/PT sub-file container (Ohana3DS port)
  gf_model.rs            # Sun/Moon-era GfModel + PICA command reader (port;
                         # XY itself uses CGFX+BCH — see PLAN.md Phase 2)
  obj.rs                 # Wavefront OBJ (+MTL for BCH) export
xy-extraction/xy-script/src/
  textfile.rs             # Gen6 text banks (XOR line crypt + VAR codes, pk3DS port)
xy-extraction/xy-script/tests/
  textfile_golden.rs       # golden vs Source_Extracted/garc/a/0/7/* (skips if absent)
xy-extraction/xy-gfx/tests/
  smdh_golden.rs           # icon pixels vs Source_Extracted/icon.png
  bcres_golden.rs          # model container vs Source_Extracted/garc/a/0/0/1
  cgfx_golden.rs           # CGFX textures/materials vs garc/a/0/0/7 (0001/0004)
  bch_golden.rs            # BCH models/textures vs garc/a/0/0/7 (pm0001_00)
xy-decomp/ghidra/
  DecompileAll.java        # Ghidra headless post-script: export all C functions
                           # (force-creates module_entry for zero-function stubs)
xy-heatmap-py/
  heatmap.py                  # CLI entry: build | serve --watch | summary | roms
  xhmap/                      # cli / collect / romdetect / page / server modules
  shell/rebuild_v2.sh         # full clean-slate re-dump of .Data/Source_Extracted
xy-extraction/xy-curate/src/
  lib.rs                 # `xy curate` pipeline (stages, manifest, validation,
                         # progress + cancellation) — real files only, never
                         # symlinks; CLI calls xy_curate::build
  manifest.rs            # read manifest.tsv / validation.tsv back (provenance)
  gfx_convert.rs         # pokemon stage: 8-file families, OBJ+MTL + PNG conversion
  csharp.rs              # C→C# deobfuscation pipeline (see its section above)
xy-extraction/xy-convert/src/
  lib.rs                 # FileKind detection (magic + GF-container probing)
  model.rs               # SceneModel intermediate (BCH/CGFX/PC -> meshes/
                         # materials/bones/decoded textures) + texture collection
  convert.rs             # OutputFile conversions: GLB/PNG/WAV/JSON bytes
  inspect.rs             # kind/magic/size/CRC32/FNV-1a/SHA-256 file info
  media.rs               # ffprobe JSON parsing + ffmpeg decode/convert/frames
  render.rs              # headless z-buffered rasterizer (textured|material|
                         # wireframe|normals|skeleton; turntable) -> PNG
xy-extraction/xy-gfx/src/gltf.rs  # glTF 2.0 (GLB) export: GltfScene neutral input,
                         # from_bch/from_cgfx mappers, embedded PNG textures,
                         # skin + inverse-bind matrices (Gauss-Jordan inverse),
                         # TRS bone nodes (quat-from-euler parity-tested vs the
                         # matrix path) + CANM skeletal-animation channels
xy-extraction/xy-convert/src/glb_check.rs  # GLB validator (gltf crate): schema +
                         # accessor/image/skin/animation decoding — the
                         # "will Blender load it" gate; wired into
                         # `xy convert glb --validate` and `xy inspect *.glb`
xy-extraction/xy-battle/        # stub — stats / encounters / trainers (data now reachable: a/2/1/*)
```

### Conventions (enforced in review)

- **Findings are written down.** Any durable discovery lands in `docs/` per
  the Documentation duty — behavior changes merged without their doc entry
  are incomplete work.
- **Parsers return typed errors** (`xy_core::Error` via `thiserror`; the gfx
  crate wraps it with a png variant) and never `unwrap()`/`panic!` on bad
  input.
- **No `unsafe`.** Parsing is bounds-checked via the `*_le`/`u8_at` slice
  helpers in `xy-core/src/util.rs`; reuse that pattern for new formats.
- **Borrowing rule:** parsers borrow the input buffer (`Bcwav<'a>` borrows
  `&'a [u8]`). In the CLI, parse+decode in one scope and keep only owned
  output (`Vec<u8>`/`String`). Never `Box::leak` to fake lifetimes.
- **Validate parsers against real assets** before merging: decode a real file
  and sanity-check (duration, sample rate, peak level, not-silence), plus a
  byte-parity golden test where a reference exists (see
  `xy-extraction/xy-audio/tests/bcwav_golden.rs`; must skip cleanly when the asset
  is missing).
- Conversions must be **byte-for-byte reproducible** against the reference
  implementations we ported from (see Provenance below).

### Validated parity status

- RomFS extraction: byte-identical to `ctrtool --romfsdir` output (653 files).
- Manual pipeline: byte-identical to the Python prototype (3663 assets).
- Banner CWAV DSP-ADPCM: byte-identical to the vgmstream-port reference WAV.
- SMDH icon: pixel-identical to an independent Python port of **Azahar's**
  `SMDH::GetIcon` (fixed 2026-10-11 — the original decoder read the large
  icon linearly from the small-icon slot at 0x2040 and produced noise that
  a same-mistake PIL reference "validated"; real layout = two Morton-tiled
  RGB565 icons at 0x2040 (24×24) / 0x24C0 (48×48)).
- GARC v4 + LZ-0x11: pk3DS `GARC.cs`/`LZSS.cs` semantics, validated by
  self-describing cross-checks and a 255-GARC corpus sweep (zero failures,
  ~4,600 payloads decode to their exact declared size); Kuriimu2 refuses
  XY's v4 GARCs, so independent byte-parity is still open.
- CRO0 parser: structural validation on **all 134 files** (header fields,
  segment table consistency, symbol/import tables); CROs are plaintext —
  the earlier "encrypted" theory is dead (see Format knowledge).
- Gen6 text decoder: pk3DS `TextFile.cs` port; golden pins on ENG/JPN
  species + item banks; 4,600/4,600 real banks convert cleanly.
- PkmnContainer: Ohana3DS `PkmnContainer.cs` port, validated against the
  real Bulbasaur PC (11 entries; BCH/CGFX magics at computed boundaries).
- PICA textures: Ohana3DS `TextureCodec.cs` port; golden-pinned against
  the `star` effect texture (40 bright/3431 dark px) + Bulbasaur body
  texture (256×256 rgba8, green-dominant).
- BCH (CtrH3D): SPICA `Formats/CtrH3D` port, validated against an
  instrumented SPICA run on the real files + a Blender render of
  pm0001_00; **981/981 model PCs decode** (722 species), goldens pin
  mesh/bone/material/texture counts.
- GLB export (2026-10-06): `xy convert glb --textures --validate` on
  pm0001_00 produces a file that **Blender 4.x imports** (8 meshes,
  71-bone armature, 5 textured materials, embedded images — headless
  import + Cycles proof render in `exports/glb/0001_Bulbasaur_00/`),
  and `glb_check` (gltf crate) validates. Blender's *"not a valid
  Blender file"* error on GLB = using File→**Open**; GLB comes in via
  File→**Import → glTF 2.0**. Census finding: XY CGFX carry **zero
  CANM** anywhere (a/0/0/7 + a/0/3/3 swept: only TXOB/MTOB/CNOD) —
  the animation exporter is exercised by synthetic round-trip tests
  and covers ORAS-style CANM, but XY motions remain the v2 GFMotion
  open RE project.

### Adding a new tool

1. New module `xy-extraction/<domain>/src/<format>.rs`: `parse()` → typed struct,
   then a `decode()`/`extract()` method. Document the binary layout in the
   module docs with an offset table. Pick the domain crate that matches the
   CLI group the command will live under.
2. Unit tests in-module (header round-trip, error paths, codec math).
3. Subcommand (or group variant) in `src/main.rs` (clap derive), wires lib →
   output files. Print a one-line summary per file: `input -> output (params)`.
4. Golden-file test in the owning crate's `tests/` if a real asset exists
   under `.Data/Raw_Extracted/` or `.Data/Source_Extracted/`.
5. Log the findings: format quirks, dead ends, parity status go to the
   matching `docs/` folder file (see Documentation duty) and the file gets
   listed in that folder's README index.

## Format knowledge

### GARC v4 + LZ-0x11 payloads — implemented in `xy-archive/src/garc.rs` + `lzss.rs`

- On-disk magics are **reversed**: `CRAG`, `OTAF`, `BTAF`, `BMIF`
  (sniffable via `CRAG`). v4 = Gen 6 (0x0400); v6 (0x0600) is rejected.
- Full offset table lives in the `garc.rs` module docs — keep in sync.
- FATO entry offsets are relative to **FATB + 0xC**; subentry start/end
  are relative to the FIMB payload start (= header data offset). `end`
  includes 0xFF padding to 4; `len` is the real length.
- Vector bits are LSB-first and **sparse** (bit 7 without bits 1–6 exists);
  `subfile()` takes the bit id, not a position.
- Payload LZ-0x11 uses the **dsdecmp indicator mapping** (S≥2: 2-byte,
  S==0: 3-byte +0x11, S==1: 4-byte +0x111) — *not* the Nintendo LZ11
  mapping in `xy-core::lz10` (whose LZ11 branch is unvalidated; do not
  reuse it here).
- First byte `0x11` does **not** imply compression (`btl_ai.garc` file 3);
  always go through `lzss::try_decompress` and keep raw bytes on failure.

### Pokémon graphics — `a/0/0/7` file families (mapped 2026-10-06)

- `a/0/0/7` = **8-file families per Pokémon form**, group id g = files
  `8g..8g+7`: `{?, PC effects+shaders, CGFX textures, BCH (anims),
  PC hi/lo **models** (= 8g+4), PF **motion** BCHs, PT, PT}` (composition
  varies slightly; models are always at +4). Forms consume extra groups
  (Venusaur: 00@2, 01@3, 51@4; Charizard: 00@7, 51@8, 52@9).
- **`a/0/0/6` is a u16 table of form-group ids** (first values
  0,1,2,4,5,6,9,10,… — ids skipped belong to the previous species's
  extra forms), but the exact species→group indexing has unresolved
  skips beyond species 3; **do not use it for naming**. The
  authoritative name is the model BCH's own `pm####_form` — the full
  sweep (981/981 model PCs, 722 species) read names that way. The old
  "8,120 slots, Bulbasaur→1" reading was wrong, and so are the
  `curated/pokemon/models` names derived from it (0001.bin is the
  effects/shaders PC, not Bulbasaur — `xy curate` needs the fix).
- Form suffixes seen: `_00` regular, `_01`/`_51`/`_52` megas,
  `_11`–`_38` other forms.
- **pm-id ≠ dex for Kalos species**: XY-internal ids 1-649 = national
  dex (render-verified), but ids ≥650 are XY-internal Kalos ids with
  no uniform dex mapping (render checks: pm0722 = Chespin = dex 650
  +72, yet pm0701 is a big-mane quadruped, NOT Hawlucha=dex 701; ids
  0650-0700 don't exist; ids up to 0774 do). `xy curate` therefore
  labels only 1-649 with species names and keeps raw pm ids beyond.
- PC entries of model files: 0 = high-poly BCH, 1 = low-poly ("_shadow")
  BCH. PF containers hold BCH **animation** files (not yet decoded —
  see Phase 2 parked items). PT containers hold texture BCHs.
- pk3DS `Legal.cs Model_XY` names 14 story-trainer model indices (Shauna,
  Tierno, Calem, Serena, …) for trainer-model naming later.

### Pokémon models — BCH (CtrH3D) port — implemented in `xy-gfx/src/bch.rs` ✅

Port of gdkchan's **SPICA** `Formats/CtrH3D` (cloned from GitHub during
development; Ohana3DS `BCH.cs` never handles XY's revision-7 files).
Validated end-to-end: Bulbasaur `pm0001_00` renders correctly in Blender
(mesh + skeleton + materials), body texture matches the in-game art.

- Container: v7 header (v≥0x21 adds RawExt, v≥8 adds flags), **relocation
  table** (`{target<<25|source<<29|ptr25}`, `*field += base(target)`,
  ptr<<2 unless target=Strings; RawDataIndex16 base = raw|1<<31 marks
  16-bit index buffers) applied **before** parsing — post-relocation all
  pointers are absolute file offsets.
- Refs: lists `{u32 ptr, i32 count}`, single objects `{u32 ptr}`;
  `[Inline]` classes (dict, material, mesh, submesh, bone) serialize in
  place; models/textures/anims/material-params are separate objects.
- Dicts: 15 inline `{list ref, tree ptr}` (models, materials, shaders,
  textures, luts, lights, cameras, fogs, skel-anims, mat-anims,
  vis-anims, light-anims, cam-anims, fog-anims, scenes); values are
  `{ptr}` refs (models/textures) or inline objects (materials/meshes/
  bones).
- Model: flags u8, bone-scaling u8, silhouette u16, world 3×4 f32,
  materials DICT (inline), meshes list (inline), 4 layer lists (Range
  end-delimiters, redundant), cullings, skeleton DICT (inline bones:
  flags, parent i16+pad, S/R/T, inverse 3×4, name), node-visibility,
  name, node-count, node patricia tree (12-byte nodes), user-address.
- Mesh: PICA command buffers (enable/submesh/disable) → attributes
  (formats 0x201/2, buffer 0x203-5, permutation 0x2bb/c, fixed
  attributes 0x232-5, uniforms 0x2c0-8 as **float24**: uniform 6 =
  position offset, 7 = pos/normal/tangent/color scales, 8 =
  uv0-2/weight scales); vertices = raw buffer (address from 0x203,
  stride from 0x205, count = max submesh index + 1), attributes
  2-byte aligned inside the stride; submeshes: 20×u16 bone indices +
  index buffer (0x227 bit31 = u16, 0x228 count, primitive = 0x25e>>8:
  0 triangles, 1 strip, 2 fan).
- Textures: TEXUNIT0_DIM 0x082 (h|w<<16), ADDR1-6 0x085-8a; PICA
  format ids map 1:1 onto `texture::TextureFormat`.
- **Animations** (dicts 8..13 = skeletal/material/visibility/light/
  camera/fog): anim = {name ptr, flags u8 (bit0 looping), type u8,
  curves u16, frames f32, element list, meta}; element = {name ptr,
  target u16, primitive u16, content inline}; content (Float 0 /
  Vector2D 2 / Vector3D 3 / Boolean 8) = flags u32 (bit N constant,
  bit 8+N inexistent per axis) + per axis {constant f32 | group ptr};
  group = {start f32, end f32, pre u8, post u8, curve u16, payload
  ptr}, payload = {start, end, interp u8, quant u8, count u16,
  invDuration, valueScale, valueOffset, frameScale, keys ptr} — keys
  use the 8 H3D quantizations (Hermite128/64/48, UnifiedHermite96/48/
  32, StepLinear64/32; FP constants /256 and /32), apply
  frame*=frameScale, value=value*valueScale+valueOffset. Validated:
  Charizard `pm0006_00_loop01` fire UV scrolls decode exactly
  (FireCoreA/FireStenA texcoord wraps).
- **XY's real skeletal motions are NOT BCH**: they live in `a/0/9/3`
  (945) + `a/0/9/4` (579) as version-2 GFMotion bitstreams (magic
  `02 00 09 00`, name-less quantized bit-packed data) that NO public
  tool decodes — SPICA's GFMotion parser handles only the ORAS v6
  format (`0x00060000`). The PF containers' BCHs hold only per-form
  material/visibility anims (mouth01 70 frames, eye01, loop01 200
  frames) with empty element lists (the curves live in the v2
  bitstream). Cracking v2 motions = open RE project (see PLAN.md).
- CLI: `xy gfx parse-model` (BCH → OBJ+MTL per model; PC containers
  route BCH entries; animations listed), `xy gfx texture`
  (BCH/CGFX/PC/PT → PNG).
- Goldens: `bch_golden.rs` (pm0001_00 hi/lo, star texture, PT texture
  pack, per-form-group rule, corpus sweep, PF material/vis anims).

### Pokémon textures — PICA decoder — implemented in `xy-gfx/src/texture.rs` ✅

Port of Ohana3DS `TextureCodec.cs`: all 14 PICA formats (rgba8…etc1a4),
8×8 Morton `TILE_ORDER`, 3DS ETC1 block scramble (64-bit halves byte-
swapped vs standard ETC1), ETC1/A4 decode with the intensity LUT.
The intermediate buffer is BGRA (matching the C# `Format32bppArgb`
writes) with one final BGRA→RGBA swap — keep that parity.
Validated: Bulbasaur body texture is pixel-plausible (green skin,
teal spots), star effect decodes to a bright starburst.

### BCRES (`CGFX`) — textures/materials in `xy-gfx/src/cgfx.rs`, mesh variant parked

Port of Ohana3DS `CGFX.cs` for **textures (TXOB), materials (MTOB),
skeletons (SOBJ), skeletal animations (CANM)** — validated against
`a/0/0/7/0001.bin` (star + shader bundle) and `0002.bin` (es001_ca).

- DICT header is 0x1c bytes (root node included); entries are radix
  nodes `{i32 ref, u16 l, u16 r, rel name, rel data}`. All pointers are
  self-relative to the field's own position (0 = null).
- **XY effect/world models are rev-9 GameFreak variants** — recon
  2026-10-06 (all XY CGFX: a/0/0/7 effects, a/0/3/3 world models):
  objects = GfxMesh records (TypeId `0x01000000`, SOBJ magic, 0x88-byte
  stride, name = node name) and shapes = TypeId `0x10000002` SOBJ
  (0x80-byte stride, name + meta + one rel-pointer to a bbox/flags
  block). Geometry is NOT in the shape records — it hangs off **CNOD
  chunks** (TypeId `0x40000001`, one per object) whose header holds
  nested self-relative descriptor tables (13-entry table → per-resource
  blocks → PICA-attribute-like descriptors with offset triples
  0x980/0xa80/0xb40…). SPICA itself throws EndOfStream on these; no
  public tool decodes them. `Cgfx::parse` skips the shapes/objects and
  sets `Model::geometry_partial` — materials/textures/anims decode
  fine. Cracking CNOD is an open RE project (see PLAN.md).
- Legacy note (wrong, kept for history): the old claim that PC entry 1
  = "CGFX model" of the mon — entry composition is per-family, see the
  corrected `a/0/0/7` section above.

### Gen 6 text banks — implemented in `xy-script/src/textfile.rs`

Port of pk3DS `TextFile.cs` (read side). Per-line XOR crypt: key
`0x7C89 + 0x2983·i`, rotate-left-3 per u16 unit. `0x0010` = variable record
`{count, code, args}` with the XY variable table (TRNAME/PKNAME/…).
Controls: `0xBE00` `\r`, `0xBE01` `\c`, `0xBE02` `[WAIT n]`, `0xBDFF`
`[~ n]`. `E07F/E08D/E08E/E08F` → nbsp/…/♂/♀. Banks: `a/0/7/*` gametext
(langs jpn, jpn2, eng, fre, ita, ger, spa, kor at 2..9 — bank index map per
pk3DS `TextReference`: 013 moves, 034 abilities, 080 species, 096 items …),
`a/0/8/*` storytext (0 = hiragana-JP, 1 = kanji-JP, 2..7 = eng..kor).
Item line 0 is the in-game "???" placeholder; species line 0 is "Egg".

### BCRES (`CGFX`) — v1 inspection in `xy-gfx/src/bcres.rs`

Header: magic, BOM `0xFEFF` (LE read), header size `0x14`, version stamp
`0x05000000`, total size (validated == file len), content count, then
`DATA`. Sub-chunks census by 4CC scan (DICT/CFLT/CALT/CENV/CCAM/CANM/MODL/
TXPT…). DICT name pools are NUL-terminated strings (NOT u16-prefixed —
that's old BCH). DICT entry layout itself is **unmapped** — recon before
mesh/material parsing. Full glTF export = Phase 2.

### CRO code modules — implemented in `xy-archive/src/cro.rs` ✅

**Plaintext — earlier "encrypted" notes were WRONG** (corrected
2026-10-05): the 0x00–0x7F region is four SHA-256 hashes (looks random),
`CRO0` sits at **0x80**, files end with `0xCC` padding to 0x1000, entropy
is non-uniform — never AES. The day it broke: an entropy histogram +
known-plaintext check (`CRO0`@0x80, `file_size`@0x90 XOR to zero keys).
boot9/otp/firm0 were **not needed** (kept gitignored for future crypto).

- Parser: full header, segment table (segment-offset = bits 0-3 index,
  4-31 offset), named exports (real symbols!), import modules (the game's
  DLL graph), relocation counts. Validated on all 134 files.
- `static.crs` = the static module's symbol table; its segment "offsets"
  are **code.bin virtual addresses** (not file offsets) — its 461 named
  exports symbolicate the main-binary decompilation (Thumb bit! match on
  `addr & !1`); 78 import modules = every game subsystem by name.
- CLI: `xy cro info` / `xy cro extract` (segment bins at load base
  0x00100000, symbols.tsv, imports.tsv). All modules extracted to
  `Source_Extracted/cro_extracted/<Name>/`.
- Ghidra: `DllBattle` battle engine imported + decompiled (same pipeline
  as code.bin); remaining 132 modules queued via the same command shape.
- Format spec: 3dbrew "CRO0" (offset table in the module docs).

### E-manual BCMA container — implemented in `xy-archive/src/bcma.rs`

A DARC-family archive that is **not** the standard 24-byte-node CTR DARC:
12-byte nodes `[flags|name_id, a, b]` (dir flag = bit 24), root's `b` sizes the
node table, names are UTF-16LE with **byte-offset** name ids into the string
table, and file data offsets are absolute. Names may be suffix-shared (dupes
possible — CLI dedupes). Each page payload is Nintendo **LZ10** wrapping an
inner DARC. Full offset tables are in the `bcma.rs` module docs — keep them in
sync with the code.

### Banner CWAV (`.bcwav`) — implemented in `xy-audio/src/bcwav.rs`

Two layout variants, auto-detected: reference-table (standard CWAV) and
chunked (bannertool/GYTB banners). Key facts:

- Standard variant: header file-size field must equal the real file length
  (validated).
- Chunked INFO: codec (2 = DSP-ADPCM), sample rate (32728 Hz = 3DS DSP
  native), total samples, channel count.
- Channel layout is **sequential** (not interleaved): per-channel sample
  regions, per-channel 16×i16 ADPCM coefficient sets.
- DSP-ADPCM: 8-byte frames → 14 samples; `scale = 1 << (header & 0xF)`,
  `coef_index = (header >> 4) & 0xF` (clamp 7); high nibble first;
  `s = ((nib*scale) << 11 + 1024 + c1*h1 + c2*h2) >> 11`, clamp to i16.
  Ported from vgmstream `src/coding/ngc_dsp_decoder.c` — the JS/Python ports
  in this repo's history match it bit-exactly.
- Known golden values (XY banner jingle): 32728 Hz, 2 ch, 95372 samples,
  ≈2.91 s, ch0 data `[0x120:0xD620)`, ch1 `[0xD620:0x1AB08)`.

### CCI/NCCH/RomFS — implemented in `xy-archive/src/romfs.rs`

- CCI: 0x100-byte RSA signature **before** the `NCSD` magic (check 0x100!);
  partition 0 descriptor at 0x120, sectors × 512 B.
- NCCH offset/size fields are in **0x200 media units**: RomFS offset/size at
  NCCH+0x1B0/0x1B4 (plain @0x190, logo @0x198, ExeFS @0x1A0).
- IVFC header (size u32 @0x58) is followed by the master hash + padding; the
  RomFS header (u32 fields, not u64!) is located by structural scan.
- Only decrypted images (Crypto Key: None) — encrypted partitions are
  detected and rejected with a clear error.

### Not parsers (by design)

- `.aac` BGM = standard ADTS; `.moflex` = Mobiclip; both belong to ffmpeg.
  If batch-convert tooling is ever wanted in Rust, prefer shelling out or a
  dedicated crate over reimplementing codecs.

## Environment notes

- Linux; ROM data under `.Data/` came from 3DS decryption done outside this
  repo (dump is decrypted — NCCH `Crypto Key: None`, no `boot9.bin` needed).
- External tools: `ctrtool` (system pkg; no CRO support), `ffmpeg`,
  `vgmstream-cli` (AUR), `3dstool` (AUR), `makerom`, **Ghidra 12.1.2 at
  `/opt/ghidra`** (headless `support/analyzeHeadless`; requires **absolute
  paths** — relative paths abort with "Path element starting with '.'"),
  `radare2`, `blender`, `python3` + PIL.
- This repo has git history — restructures land as tracked renames; ask
  before bulk-moving anything outside this repo and
  `.Data/Source_Extracted/`.
- Kuriimu2 1.2.1: `DOTNET_ROLL_FORWARD=LatestMajor`, copy inputs to /tmp
  (mangles `/mnt` paths), and its GARC4 plugin rejects XY's v4 GARCs.
