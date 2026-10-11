# PLAN.md — PokeXYZ-RDComp roadmap

Living plan for the Pokémon X/Y decompilation toolkit. Status markers:
✅ done and validated · 🔨 in progress · ⬜ planned.

## Current state (validated)

| Module | Format | Parity reference | Status |
|---|---|---|---|
| `xy-archive/src/romfs.rs` | CCI/NCCH/IVFC → RomFS | `ctrtool --romfsdir` (653 files) | ✅ byte-identical |
| `xy-archive/src/bcma.rs` + `xy-core/src/lz10.rs` | XY e-manual (BCMA→LZ10→DARC) | Python prototype (3663 assets) | ✅ byte-identical |
| `xy-audio/src/bcwav.rs` | CWAV (both variants) + DSP-ADPCM | vgmstream port (reference WAV) | ✅ byte-identical |
| `xy-gfx/src/smdh.rs` | icon + title metadata | PIL reference PNG | ✅ pixel-identical |
| `xy-archive/src/garc.rs` + `lzss.rs` | GARC v4 + LZ-0x11 (dsdecmp) | pk3DS semantics + 255-GARC sweep, ~4,600 payloads to exact declared size | ✅ (independent byte-parity open) |
| `xy-archive/src/cro.rs` | CRO0 code modules | 3dbrew spec; validated on all 134 files | ✅ structural |
| `xy-script/src/textfile.rs` | Gen6 text banks → JSON | pk3DS `TextFile.cs` port; golden pins (Bulbasaur/Greninja/フシギダネ) | ✅ |
| `xy-gfx/src/bcres.rs` | BCRES/CGFX v1 inspection | header/total-size self-checks | ✅ v1 |
| `xy-gfx/src/pkmn.rs` | GF PC/PF/PT sub-file containers | Ohana3DS `PkmnContainer.cs` port; validated vs Bulbasaur PC | ✅ |

Toolchain gates: `cargo test` **65/65** · `clippy -D warnings` clean ·
`fmt` clean · release profile (LTO + strip).

## Phase 0 — codify what's only manually verified ✅ (2026-10-05)

All three golden tests landed and pass against the real assets:
`romfs_golden.rs` (hash manifest vs ctrtool tree), `manual_golden.rs`
(counts + per-file hashes), `smdh_golden.rs` (pixel compare vs PIL PNG).

## Phase 1 — GARC archives (the 1.5 GB prize) ✅ (extraction complete)

`Data/extracted/romfs/a/` holds the game's real content: maps, models,
textures, scripts, encounter data — inside Gamefreak GARC containers.

1. **Recon (Kuriimu2 CLI).** ✅ Done 2026-10-05 — with a twist: Kuriimu2
   1.2.1's GARC4 plugin refuses to load XY's v4 GARCs ("No plugin could
   identify the file"; runs only with `DOTNET_ROLL_FORWARD=LatestMajor`, and
   mangles `/mnt/...` paths). Reference is pk3DS `GARC.cs` semantics +
   the container's self-describing cross-checks instead (see garc_golden).
2. **Document the layout** ✅ in `crates/xy-archive/src/garc.rs` module docs
   (magic `CRAG`, OTAF/BTAF/BMIF, verified against `a/0/0/1` + `btl_ai.garc`).
3. **Implement `garc.rs`** ✅: `parse()` (validates fileSize, FATO↔FATB
   consistency, FIMB bounds, per-subentry ranges), `subfile()` by vector
   bit id. Corpus check: **255/255 GARCs under `romfs/a/` (< 6 MB each)
   parse and extract cleanly.**
4. **Inner compression** ✅: `crates/xy-archive/src/lzss.rs` — LZ-0x11 in
   the **dsdecmp indicator mapping** (S≥2: 2-byte; S==0: 3-byte +0x11;
   S==1: 4-byte +0x111), ported from pk3DS `LZSS.cs`. Empirically pinned:
   ~4,600 subfiles decode to their exact declared size, with coherent
   content (`.bclan` banks with filenames, "MM" text tables, CGFX models).
   `try_decompress` keeps raw bytes on failure (pk3DS try/catch semantics;
   `btl_ai.garc` file 3's coincidental `0x11` start stays raw — golden-
   tested). NOTE: `xy-core::lz10`'s LZ11 branch follows the *Nintendo*
   mapping and has never seen real data — do not reuse it for GARC payloads.
5. **Parity gate** 🔨 partial: Kuriimu2 byte-parity is blocked (see 1);
   golden tests cover container self-consistency + CGFX embedded-size
   cross-check + pinned decoded bytes. Full byte-parity vs an independent
   extractor remains open if a working tool is found (EveryFileExplorer).
6. **CLI** ✅: `xy archive list <file>` + `xy archive unpack -i <file>
   -o <dir> [--decompress]`, pk3DS-compatible names (`N.bin` / `N/NN.bin`).

Remaining in Phase 1: GARC **repacking** (insertion/`PackGARC` port) —
needed before any game-editing use case, not needed for extraction.

Known content map (verified with `xy archive list` on 2026-10-05; sizes
are of the GARC itself): the 1.48 GB splits as `a/0` 1.3 GB / `a/1` 231 MB /
`a/2` 12 MB across 271 containers. Heavyweights: `a/0/0/7` **993 MB,
7,852 subfiles, CGFX models — the actual Pokémon graphics archive**
(the earlier "a/0/9/3 ≈ Pokémon models" note was wrong: that GARC is
0.3 MB of small `0200`-magic records), `a/1/6/5` 97 MB (734 CGFX),
`a/0/3/3` 82 MB (1,779 CGFX, world models), `a/0/4/1` 63 MB (1,030 data
files), `a/0/1/3` 49 MB (171 files). Text banks: `a/0/1/0-9` BCLAN,
`a/0/2/1` "MM" story text (286 banks). Encounters: `a/1/5/9` (241 zones).

## Phase 2 — texture/model conversion 🔨

- [x] `bcres.rs` v1 inspection (header, chunk census, names) — 2026-10-05.
- [x] **PkmnContainer port** — `xy-gfx/src/pkmn.rs`: `"PC"/"PF"/"PT" +
      u16 count + (count+1) × u32 end-offsets` (stride 4 — entry i spans
      [end[i-1]|header, end[i])). Unit test caught the stride-8 bug against
      Ohana3DS semantics; validated vs Bulbasaur `a/0/0/7/0001.bin`
      (11 entries, BCH/CGFX magics at boundaries).
- [x] **PC contents decoded** (2026-10-05, Bulbasaur): entry 0 empty;
      **entry 1 = CGFX model container (51 KB)**; entry 2 = BCH skeleton;
      entries 3–6 = CGFX (textures/more); entries 8–10 = small BCH anims.
      XY-era models = **CGFX (BCRES) + BCH** — real port targets are
      Ohana3DS `CGFX.cs` (1,485 ln) + `BCH.cs` (2,171 ln) from
      `Tools & Libs/Ohana3DS-Rebirth-Fork` (also `GfTexture.cs`/`DMP.cs`
      for PICA textures incl. ETC1, `GfMotion.cs` for anims).
      *(2026-10-06 correction: 0001.bin is the species-1 effects/shaders
      PC — the mon MODELS live in the family model PC 0004.bin.)*
- [x] `gf_model.rs` — Sun/Moon-era GfModel + PICA command-buffer reader
      port (kept for future titles; XY predates this format) + `obj.rs`
      OBJ export + `xy gfx parse-model` router.
- [x] **Texture decode** ✅ (2026-10-06) — `xy-gfx/src/texture.rs`:
      Ohana3DS `TextureCodec.cs` port. All 14 PICA formats, 8×8 Morton
      TILE_ORDER, 3DS ETC1 scramble + LUT decode, BGRA-intermediate
      parity. CLI `xy gfx texture` (BCH/CGFX/PC/PT → PNG). Validated:
      Bulbasaur body texture pixel-plausible, star effect pinned.
- [x] **CGFX port (partial)** ✅ (2026-10-06) — `xy-gfx/src/cgfx.rs`
      (Ohana3DS `CGFX.cs` port): DICT radix trees (0x1c-byte header),
      TXOB textures, MTOB materials (colors + PICA command buffers +
      texture-mapper names), SOBJ skeletons, CANM skeletal animations.
      DICT layout + relative-offset rules fully mapped. **Parked:** XY's
      rev-9 effect/world models link geometry through CNOD chunks with a
      non-SDK SOBJ shape layout (SPICA itself fails on them) — shapes
      are skipped via `Model::geometry_partial`; recon notes in AGENTS.md.
- [x] **BCH port** ✅ (2026-10-06) — `xy-gfx/src/bch.rs`: port of
      gdkchan's **SPICA** `Formats/CtrH3D` (Ohana3DS `BCH.cs` cannot
      handle XY's revision-7 files). Container + relocation table +
      patricia dicts + models/meshes (PICA command buffers, float24
      uniforms, attribute decoding, strips/fans)/materials/bones/
      textures. CLI `xy gfx parse-model` → OBJ+MTL. **Validated end to
      end: Bulbasaur pm0001_00 renders correctly in Blender (mesh +
      skeleton); 981/981 model PCs decode (722 species).**
- [x] **`a/0/0/7` layout cracked** (2026-10-06): 8-file families per
      form, model PC at family+4; `a/0/0/6` = form-group table (exact
      species→group skips unmapped — name from model BCHs, not the
      table; `curated/pokemon/models` naming is stale and needs the fix).
- [x] **BCH animations ported** (2026-10-06 session 2) — dicts 8..13,
      element/content/keyframe layouts traced via instrumented SPICA;
      all 8 H3D quantizations. Charizard's `pm0006_00_loop01` fire UV
      scrolls decode exactly. **Finding:** XY's real skeletal motions
      are NOT BCH — `a/0/9/3`+`a/0/9/4` hold name-less version-2 GFMotion
      bitstreams (`02 00 09 00`) no public tool decodes (SPICA = ORAS
      v6 only); PF BCHs hold per-form material/vis anims whose element
      lists are empty (curves live in the v2 stream).
- [x] **Curate naming fixed** (2026-10-06 session 2) — model families
      named from each BCH's own `pm####_form`; render-verified that
      XY-internal ids 1-649 = national dex (Bulbasaur/Charizard/
      Chespin+72 checks) and that ≥650 does NOT map to dex (pm0701 ≠
      Hawlucha) → species labels only for 1-649, raw pm ids beyond;
      stale-link cleanup on re-run; real `curated/` regenerated (977
      model PCs).
- [x] **Curated pipeline rebuilt** (2026-10-06 session 3) — `xy curate` is
      now the final extract → rebuild → convert → organize layer:
      **real files only (no symlinks)**, deterministic (sorted iteration,
      stable names; re-runs hash-identical for `manifest.tsv`,
      `validation.tsv`, OBJ/PNG/JSON/C# outputs), 86,218 manifest rows +
      86,218 validation checks (0 failures) over a 7.4 GB tree. Conversions:
      all 981 `a/0/0/7` families organized (`pokemon/families/`), models →
      OBJ+MTL (3,908 files), textures → PNG (17,592 files), text banks named
      per pk3DS TextReference (`text/organized/`), `code/modules.json`
      dependency graph, CRO+Ghidra corpus under `code/`; unmapped data
      preserved raw under `unknown/` (24,677 files). See AGENTS.md
      "Curated extraction".
- [x] **C → C# deobfuscation pipeline** (2026-10-06 session 3) —
      `src/curate/csharp.rs`: mechanical C→C# conversion of all 38,384
      Ghidra functions (value ints → long, pointer widths kept, CONCAT/
      stack/label/char-literal/deref-assign/comma-expression rewrites,
      thunk `params` widening, stubs carrying the original C).
      15,197/38,384 auto-converted corpus-wide; dotnet-10 compile checks:
      DllBattle 12 errors / 3,011 functions, DllBag 34 / 454. Deterministic
      (hash-verified re-runs).
- [ ] Shader binaries: `a/0/0/7` holds BCRES with DMP shader objects
      (SHDR/DVLB/DVLP/DVLE, `.vsh` names) — inventory + (stretch) disasm.
- [ ] XY rev-9 **CNOD mesh variant** recon (effect/world CGFX models) —
      recon done 2026-10-06: GfxMesh objects (TypeId 0x01000000) +
      TypeId-0x10000002 shapes hold no geometry; it hangs off CNOD
      chunks' nested self-relative descriptor tables. SPICA also fails
      (EndOfStream). Open RE project.
- [ ] **XY v2 motion bitstreams** (`a/0/9/3` + `a/0/9/4`, 1524 files):
      name-less quantized bit-packed skeletal curves, format unresearched
      publicly. This is the only blocker for Pokémon skeletal animation
      playback.
- [ ] `bclim.rs` — CTR texture → PNG/RGBA (289 manual textures + UI).
- [ ] `.bclyt` layout parsing (3374 manual layouts; low priority).

## Phase 3 — sound archive ⬜

- [ ] `crates/xy-audio/src/bcsar.rs` — `sound/xy_sound.bcsar` (21 MB: SEQ/
      BANK/WAV collections). The CWAV-in-CSAR entries should reuse
      `xy-audio/src/bcwav.rs` types; DSP-ADPCM decode is already done.
      Priority only after GARCs, since BGM/SFX already lives in `sound/*.aac`.

## Phase 5 — code decompilation ✅ corpus complete (naming/refinement open)

- [x] `code.bin` Ghidra pass — 2026-10-05: project + 14,864 decompiled C
      functions at `Data/src_extracted/decomp/` (see MAP.md).
- [x] **CRO modules unblocked + parsed** — 2026-10-05 correction: never
      encrypted (magic `CRO0`@0x80 after the SHA-256 hash block; entropy
      histogram + known-plaintext check broke the wrong assumption).
      `xy-archive/src/cro.rs` validated on all 134 files; `xy cro info/
      extract`; all modules at `src_extracted/cro_extracted/<Name>/`;
      `static.crs` symbolicates code.bin (129/14,864 via 461 named
      exports, Thumb-bit match). boot9/otp/firm0 were never needed for
      CROs (kept, gitignored, for future crypto).
- [x] **All 133 CRO modules decompiled** — 2026-10-05 re-dump v2:
      `decomp/cro_c/<Name>/` = **23,521 functions** (incl. DllBattle's
      3,010); grand total with code.bin = **38,385 C functions**. Pipeline
      = rebuild_v2.sh (Ghidra headless, absolute paths, per-module base
      from segments.tsv, unique program imports via `cro_src/<Name>.bin`).
- [x] **Stub-module fix** — the 27 `DllTest*`/`DllDebug*`/`DllFieldDebug`/
      `DllPokeViewer` modules are 172–220-byte template stubs; raw imports
      that small get zero functions from auto-analysis (no entry points),
      which earlier runs mislabeled as transient failures. `DecompileAll.java`
      now force-creates `module_entry` at `getMinAddress()` (full body)
      when a program has zero functions → all 27 decompile to their single
      dispatcher. Gotcha: `getImageBase()` returns 0 for raw-binary
      imports — use `getMinAddress()`.
- [ ] Apply symbols inside Ghidra (postScript reading symbols.tsv; add
      cross-module symbolication from each Dll's exports/imports).
- [ ] Relocation-aware analysis (apply internal relocs pre-import for
      better xrefs).
- [ ] Fan-source organization: name functions from text banks + data-table
      cross-references (battle tables in `a/2/1/*`).

## Phase 4 — Pokémon Y rollout ⬜

- [ ] Obtain a **Y** dump (title `0004000000055E00`, `kujira-2`) — this is the
      missing game; the **X** dump (`0004000000055D00`, `kujira-1`) is already
      in `Data/` and is what all current golden tests validate against. (Fixup
      2026-10-05: this entry previously had X/Y swapped — the SMDH in our
      `0055D00` dump says "Pokémon X" in all 12 regions, matching AGENTS.md.)
- [ ] Run the whole pipeline end-to-end; the parsers are already
      game-agnostic, so this is a validation exercise: same golden checks with
      Y assets, watch for format-version drift (NCCH/IVFC/BCMA version fields).
- [ ] Keep any X/Y differences as data, not code branches.

## Housekeeping ⬜

- [ ] `git init` the parent project (AGENTS.md documents the no-history
      caveat; this unlocks real provenance for the ports).
- [ ] Decide on committing small reference fixtures (banner jingle + a few
      GARCs) so golden tests survive `Data/` moves.
- [ ] Optional: rename `Data/extracted_assets/audio` WAVs to match in-game
      labels when a name table is available from the GARCs.

## Ground rules (inherited from AGENTS.md)

- Byte-for-byte parity against a reference implementation before any parser
  is "done".
- Golden tests skip cleanly when assets are absent.
- No `unsafe`, no panics on malformed input, parsers borrow, CLI owns.
- Don't hardcode title IDs; X first, Y by validation.

## Status log

### 2026-10-11 — repo consolidation: PokeXYZ-RDComp ✅ (session 5)

The user created `/mnt/dev/Dev/Coding_Projects/PokeXYZ-RDComp/` as the one
project space for recomp + decomp, Rust-first with python helpers only.
Everything moved (same filesystem, `mv` = rename, nothing copied):

- **Rust workspace → repo root** (was `PokeXYZ-PC/XY_Decomp_Tools/`):
  `Cargo.toml`, `crates/`, `src/`, `reference/`, `target/` unchanged;
  `target/` carried over so incremental builds survive.
- **Ghidra scripts unified**: `ghidra_3ds_scripts/` + `ghidra/` merged into
  **`scripts/ghidra/`**; driver now derives paths from its own location
  (`RD_ROOT`) with env overrides (`GHIDRA_INSTALL_DIR`, `XY_PROJECT_LOC`,
  `XY_PROJECT`, `XY_SCRIPT_DIR`) so repo moves stop breaking it.
- **`Data/` → `Data/`** at repo root (24 GiB): dumps, `3ds_extracted/`,
  `src_extracted/` (incl. `pipeline/`, `recomp/port/`), plus new `ports/`
  and **`update/` (v1.5 dump slot)** with a README pinning update title
  IDs `0004000e00055d00/00055e00` and the v1.5 code.bin size
  (5,169,152 B vs v1.0's 5,156,864).
- **Pipeline tools → `thirdparty/`** (gitignored, `fetch.sh` recreates):
  3dsrecomp, zakuro, 3DS-Decomp-Pipeline, objdiff. Everything else stays
  in `../PokeXYZ-PC/Tools & Libs/` as reference.
- **recomp track**: `recomp/build.sh` (adapted from the public
  PokeXY-Recomp builder — reuses `thirdparty/` checkouts, writes port
  trees to `Data/ports/<name>/`, auto-finds the dump under `Data/`),
  `recomp/overrides/` slot for C shims.
- **helpers**: `scripts/heatmap.py` (from the public repo),
  `docs/findings.md` + `docs/ghidra-scripts.md`.
- **Path patches**: 10 golden tests `../../../Data` → `../../Data`
  (Data now sits beside `crates/`, not above the old workspace),
  `scripts/rebuild_v2.sh` re-pointed, GLB generator string updated.
- **git**: repo initialized (was none); `.gitignore` extends the console-
  key block (`otp.mem`, `firm0.bin`, …) and ignores `Data/**` +
  `thirdparty/**` with README/fetch carve-outs, plus `/exports/`.
  **Nothing game-derived or key-like is committable by construction.**
- Gates at move time: `cargo test --workspace` **104/104 green** (up from
  the documented 65 because the Data goldens now *run* instead of
  skipping — romfs/garc/manual/cro/bch/bcwav/textfile/smdh/cgfx/bcres all
  re-verified against the moved tree), `clippy -D warnings` clean,
  `fmt --check` clean.

### 2026-10-10 — recomp + matching-decomp pipelines brought online ✅ (session 4)

Goal: wire the newly organized `Tools & Libs/` toolchain into working
recompilation (Route B) and matching-decomp (Route C) pipelines.

- **Version pinning audit**: our `code.bin` is **5,156,864 bytes = v1.0**
  (SHA-256 `0f6d3e1d…`), 12,288 bytes smaller than the report's v1.5 marker
  (5,169,152). The dump is the base cart (`_v00`) and the update title is not
  on disk — **the whole project is pinned to v1.0**; all 38k Ghidra functions
  and every golden test already agree with it. Re-extract from
  `0004000e/00055d00` only if we ever deliberately switch versions.
- **Static recompilation (fearkov/3dsrecomp + Zakuro)** — Pokémon X works:
  `analyze` finds **71,163 functions** (26,493 exec + 44,670 CRO), 95.7 %
  code coverage, residual gaps = 673 B dead-ends in the executable + a 2 KB
  run in DllTitle (interpreter fallback covers them). `build` emitted
  **679 MiB of C (664 files, 529/529 units compiled, 0 hand overrides)**
  → `Data/src_extracted/recomp/0004000000055D00.so` (296 MB x86-64 ELF,
  75,452 .symtab symbols). `verify` (built with `--features verify`) ran
  **24,063 recompiled functions against Zakuro's interpreter: 0 mismatched**.
  Zakuro itself builds clean (ALSA/udev present). Both tools ~30-40 min per
  full pass. Note: upstream README lists Pokémon **Y** as tested (99.8 %),
  not just the report's claim of "X untested".
- **Ghidra 12.1.2 + 3ds-Ghidra-Scripts**: Ghidra 12 removed Jython, so stock
  `analyzeHeadless` refuses .py scripts. Solution: **pyghidra 3.1.0 + jpype
  1.7.1** (user site-packages) + driver `ghidra_3ds_scripts/xy_ghidra_3ds.py`
  (starts the JVM via PyGhidra, runs `ghidra_script()` per program; publishes
  `currentProgram`/`monitor` on `__main__` so the Jython-era
  `from __main__ import …` helpers keep working). Patched suite copies in
  `ghidra_3ds_scripts/` (CPython byte helpers, headless `askYesNo`,
  readBytes via JPype array, TLS-site try/except, graceful Handles skip).
  Ran over **all 135 programs**: code.bin got **831 SVC bookmarks** +
  TLS retypes; `Label3dsHandles` documented-skip (XY uses C-era
  `srvGetServiceHandle`, no `ctr::srv::GetServiceHandleDirect` anchor);
  `Label3dsCTRFns` low-yield for the same reason (C++-era inline patterns).
  Labeled project copy: `decomp/ghidra_project_3ds/` (original untouched).
- **Symbol CSVs**: `ExportSymbolsHeadless.py` (headless port of
  AlgebraManiacABC's ExportSymbols.java) exported all **134 binaries** in
  pipeline format (code.bin: 16,545 symbols with $a/$t modes). CRO CSVs
  shifted −0x100000 into the pipeline's text-offset coordinate frame.
- **Matching-decomp pipeline (AlgebraManiacABC/3DS-Decomp-Pipeline)** —
  configured at `Data/src_extracted/pipeline/` (orig = code.bin + 133 CROs +
  exheader.bin via ctrtool; tools = Arch arm-none-eabi gcc/ld/objcopy
  symlinked; cc.yaml bootstrap — NOT yet a matching compiler config).
  **Round-trip proven: all 134 binaries relinked from their 72,742 split
  objects recreate SHA-256-identical originals** (code.bin included after
  deduplicating Ghidra's duplicate `thunk_FUN_*` names in the CSV — fix
  applied in the symbols file; exporter `unique` mode does the same).
  `objdiff.json` (134 units) generated; `--use-splits-only --objdiff` has an
  upstream UnboundLocalError — use plain `--objdiff` (empty compile is fine).
- **XAMX (AMX layer)**: XY script GARCs per XAMX docs — global scripts
  `a/0/3/1` (**59/59 disassembled**), map scripts `a/0/1/2`, plus `a/0/9/0`,
  `a/1/0/7`, `btl_ai.garc`. Outputs in `src_extracted/xamx/` (`_`
  subfolders). **Round-trip reassembly of 17.bin is byte-identical** to the
  source file — script edits need no ARM work. Tool is interactive
  (press-enter prompts; feed stdin).
- **REA MCP**: deep providers unconfigured in this environment (needs
  GHIDRA_INSTALL_DIR / REA_PWNTOOLS_PYTHON env for the MCP server);
  `open_binary` classification used for split-object sanity checks.
- Gates: no XY_Decomp_Tools code changes this session (tooling + data only).

### 2026-10-06 (session 3) — curated pipeline + C→C# deobfuscation ✅

- **`xy curate` rewritten as the final pipeline layer**: real files only
  (symlinks removed entirely), seven stages (`data,text,pokemon,media,code,
  csharp,unknown`), `manifest.tsv` (86,218 rows: source → path → action →
  detail → bytes → status) + `validation.tsv` (86,218 checks, 0 failures:
  existence/size/magic for RIFF/ftyp/PNG/JSON/OBJ), deterministic output
  (re-run hash-identical), wipe guard (refuses to delete non-curated dirs).
- **Conversions in the tree**: 981/981 Pokémon families organized and named
  from BCH `pm####_form` names; 3,908 OBJ/MTL model files (hi+lo per
  family, Blender-loadable); 17,592 texture PNGs (CGFX +2 pack and PT +6/+7
  packs); gametext organized with pk3DS names (species/moves/abilities/
  items… + index.json per language); `code/modules.json` = segments,
  exports, import graph, decomp counts for all 134 CRO modules; full CRO +
  Ghidra corpus copied under `code/`; 24,677 unmapped files preserved under
  `unknown/` with READMEs. Total 7.4 GB / 114,530 entries, built in ~20 s.
- **C→C# deobfuscation pipeline** (`src/curate/csharp.rs`): converts the
  entire Ghidra corpus (38,384 functions, 134 modules, ~1.5 s) into C#
  `unsafe` partial classes. Iterated against the real corpus with dotnet-10
  compile checks down from 1,000+ to 12 (DllBattle/3,011 fns) and 34
  (DllBag/454 fns) residual errors: value ints widen to `long`, pointer
  widths exact, CONCAT/stack/label/char-literal/deref-assign/comma-
  expression/thunk-widening rewrites, stubs keep the original C. Known
  residue: comma-expression edge cases + multi-line call pointer args.
- Gates: `cargo test --workspace` 82/82 · clippy `-D warnings` clean ·
  fmt clean.

### 2026-10-06 (session 2) — animations + CNOD/motion recon + curate fix ✅

- **BCH animations decode**: dicts 8-13 + elements + Float/Vector2D/
  Vector3D/Boolean contents + all 8 H3D keyframe quantizations,
  traced from an instrumented SPICA run on Charizard's PF BCHs.
  `pm0006_00_loop01` fire UV-scroll curves decode exactly (matching
  SPICA element-for-element). CLI `xy gfx parse-model` lists them;
  goldens pin the fire curves + mouth/eye anim metadata.
- **Reality check on motions**: XY skeletal Pokémon motions are the
  `a/0/9/3`/`a/0/9/4` v2 GFMotion bitstreams — 1524 name-less,
  bit-packed files that no public tool decodes (SPICA's GFMotion =
  ORAS v6). PF-container BCHs hold only mouth/eye/loop material+vis
  anims with empty element lists. Documented as the main open item.
- **CNOD recon closed out**: effect/world rev-9 CGFX models carry no
  geometry in their shape records; it hangs off nested self-relative
  CNOD descriptor tables. Even SPICA throws EndOfStream. Parked.
- **Curate naming rewritten + render-verified**: families now named
  from each model BCH's own `pm####_form`; renders prove pm0001 =
  Bulbasaur, pm0722 = Chespin (+72), but pm0701 ≠ Hawlucha → species
  labels only for ids 1-649, raw ids beyond; models dir cleared of
  stale links on re-run; real `curated/` regenerated (977 files,
  was 8120 mislabeled). Gates: 71/71 tests · clippy · fmt clean.

### 2026-10-06 — Phase 2: BCH/CtrH3D + PICA textures + CGFX materials ✅

- **Bulbasaur renders.** `xy gfx parse-model` on `a/0/0/7/0004.bin`
  produces `pm0001_00.obj` (+MTL); Blender headless renders the correct
  Pokémon (head, ears, claws) — 3,499 vertices, 16,116 indices, 71
  bones across 7 meshes, plus `pm0001_00_shadow` low-poly. Textures via
  `xy gfx texture` on `0006.bin` (PT) give the named RGBA8 set
  (pm0001_00_BodyA1 256×256 …) — visually confirmed against the in-game
  art (green skin, teal spots).
- **Reference pivot:** Ohana3DS `BCH.cs`/`CGFX.cs` cannot parse XY's
  files (rev-7 BCH / rev-9 CGFX). Ported gdkchan's **SPICA** instead
  (cloned to /tmp during the session): `Formats/CtrH3D` = the BCH path;
  its CtrGfx side also fails on XY's CNOD-linked effect models (EndOfStream).
- **Recon wins:** `a/0/0/7` = 8-file families per form (models at +4,
  PF motions at +5, PT textures at +6..8); `a/0/0/6` = form-group table
  (exact skips unmapped — names must come from model BCHs; the
  `curated/` species naming is stale); PT containers = texture BCHs;
  0001.bin = effects/shaders PC (not Bulbasaur's model); the 0001.bin
  "BCH" entries are shader packages; the "anim" BCHs in 0001.bin are
  leftover Maya TestPlane assets.
- **CGFX (Ohana3DS port, partial):** DICT radix layout mapped
  (0x1c-byte header, self-relative pointers), TXOB/MTOB/SOBJ-skeleton/
  CANM decode; materials carry texture-mapper names. XY rev-9
  CNOD-linked mesh variant skipped (`geometry_partial`) — parked recon.
- **Texture codec:** all 14 PICA formats + Morton + ETC1/ETC1A4
  (Ohana3DS `TextureCodec.cs` port, BGRA-intermediate parity).
- **Corpus sweep: 981/981 model PCs decode** (722 species, forms
  00/01/11-38/51/52). Gates: 65/65 tests (6 new BCH goldens, 3 CGFX
  goldens, corpus sweep) · clippy `-D warnings` · fmt clean.
- Parked: H3D skeletal animations (PF BCHs), CNOD mesh recon,
  `curate` naming fix, BCLIM, bclyt, glTF export.

### 2026-10-05 (post-dump) — stub-module truth + full report ✅

- **Correction to the entry below**: the 27 CRO "transient Ghidra failures"
  were **never transient** — they reproduce deterministically. Those modules
  (`DllTest*` ×23, `DllDebug*`, `DllFieldDebug`, `DllPokeViewer`) are
  172–220-byte template stubs; Ghidra's auto-analysis finds no entry points
  in raw imports that small and creates zero functions, so `DecompileAll.java`
  emitted header-only manifests all along. Fix in the post-script: force
  `module_entry` at `getMinAddress()` (full-body) when a program has zero
  functions. All 27 now decompile to their single jump-table dispatcher
  (`if (param_2 < 5) return table[param_2](...)`). Gotcha pinned:
  `getImageBase()` returns 0 for raw-binary imports — use `getMinAddress()`.
- Final corpus: **133/133 CRO modules, 23,521 functions** + code.bin
  14,864 = **38,385 decompiled C functions**. `audio/` completed to
  233 WAVs (banner jingle via `xy audio bcwav`).
- `Data/src_extracted/MAP.md` rewritten as the accurate output index
  (the v2 script didn't generate it); full project report at
  `Data/src_extracted/REPORT.md`. Re-dump tooling preserved at
  `scripts/rebuild_v2.sh` (+ logs).
- Gates: 46/46 tests · clippy clean · fmt clean.

### 2026-10-05 (final) — clean-slate re-dump v2 + icon truth + GF-model groundwork ✅

- User wiped `src_extracted`; full re-dump via `rebuild_v2.sh` (all prior
  fixes folded in: ghidra_project mkdir, unique Ghidra program names,
  icon 8×, banner jingle re-slice from `banner.bin`). Prior tree had been
  preserved at `src_extracted_old` (pre-toolkit `banner/`, `icon_4x.png`,
  `manual_dec/` live there — CBMD parsing still pending).
- **Icon correction**: the SMDH decode was right all along — the icon is a
  dithered dark-navy image. Proven by cross-checking 9 update-CIA system
  icons (correlation ratio 0.39 vs validated 0.26–0.48 range) after an
  entropy/known-plaintext analysis killed the "encrypted" theory. Added
  `--scale N` to `xy gfx icon` (nearest-neighbor) + regenerated
  `icon.png` + `icon_8x.png`; smdh_golden rewritten with content anchors
  (white corner = RGB565 0xFFFF → (248,252,248) under shift expansion)
  + a not-noise correlation guard replacing the self-referential
  PNG-parity check.
- **Ohana3DS-Rebirth fork added to Tools & Libs** (user): the reference
  implementation for Phase 2 — BCH.cs (2,171 ln), CGFX.cs (1,485 ln),
  PkmnContainer/PC/GR/GfModel, DMP textures (ETC1), BCLIM, GfMotion.
  Ported to Rust so far: `xy-gfx/src/pkmn.rs` (PkmnContainer — unit test
  caught the stride-4 offset-table bug), `gf_model.rs` (Sun/Moon-era
  GfModel + PICA command reader subset), `obj.rs` (Blender-ready export),
  CLI `xy gfx parse-model`. **Key recon**: XY-era PC containers hold
  CGFX (BCRES) models + BCH skeletons/anims — so the next port targets
  are CGFX.cs + BCH.cs, not GfModel (that is the Sun/Moon-era format).
- Ghidra corpus finalized: 106/133 CRO modules + code.bin = **38,358
  decompiled C functions**; 27 stragglers re-runnable via the idempotent
  `ghidra_stage.sh` (transient project-dir issue mid-run, since fixed).
- Gates: 46/46 tests · clippy clean · fmt clean.

### 2026-10-05 (curated tree) — `xy curate` + species-named models ✅

- **`a/0/0/6` cracked** (empirically): it is the species/form → file index
  into the `a/0/0/7` graphics GARC (probe: Bulbasaur→1, Pikachu→29,
  Greninja→740, Volcanion→822; 8,120 slots for 7,852 files).
- **Model container family identified**: `PC`/`PF`/`PT` GameFreak
  containers wrap the BCH models + textures + shaders. PC layout (recon,
  needs validation before a parser): `"PC" + u16 count + u16 header size
  (0x80) + count × u32 end-offsets`, 0x80-aligned, subfile k spans
  [offset(k-1)|0x80, offset k). BCH magic confirmed at the boundaries.
  `a/0/0/7` also contains raw BCRES with DMP shader objects
  (SHDR/DVLB/DVLP/DVLE, `gfx_DefaultShader0.vsh` names) — the shader
  inventory Phase 2 wanted.
- **`xy curate`** (root-package module `src/curate.rs`): builds
  `src_extracted/curated/` — semantic dirs from the pk3DS GARC table +
  empirical additions; **8,120 Pokémon model files named** from the index
  table + ENG species bank (`0001_Bulbasaur.bin` … `0721_Volcanion.bin`,
  golden-checked names incl. Greninja/Ho-Oh/Mewtwo); per-directory
  READMEs; everything not yet mapped preserved under `misc/`; symlink-based
  (3.9 MB) with `--copy` fallback; idempotent re-runs.
- Gates: 40/40 tests · clippy clean · fmt clean.

### 2026-10-05 (final) — full-pipeline session: extraction, decomp, text, BCRES ✅

User reorganized `Data/` (`extracted` → `3ds_extracted`, `extracted_assets`
→ `src_extracted`); tests now resolve assets via `XY_DATA_ROOT` with the new
default paths.

- **Phase 0 complete**: `romfs_golden.rs` (653 files, FNV-1a manifest vs the
  ctrtool tree — caught that `walk()` offsets are IVFC-subslice-relative;
  added `Romfs::image()` accessor), `manual_golden.rs` (206 pages / 3,663
  assets, hashes; `bcma::unique_name` moved into the lib so CLI and test
  share naming), `smdh_golden.rs` (pixels vs PIL reference PNG, decoded with
  the png crate).
- **Full extraction**: new `xy archive extract-tree` (mirrors romfs paths
  under the output root, writes `manifest.tsv`) — 271/271 GARCs →
  `src_extracted/garc/` (44,512 files, 14,409 decompressed, 1.59 GB → 4.6 GB)
  + 133 CROs staged at `src_extracted/cro/`.
- **Ghidra decompilation** of `exefs/code.bin` (5.1 MB ARM11) at
  `0x00100000`: `ghidra/DecompileAll.java` post-script exported **14,864
  decompiled C functions, zero failures** to `src_extracted/decomp/code_c/`
  (+ `functions.tsv`); Ghidra project kept at
  `src_extracted/decomp/ghidra_project/`. Note: analyzeHeadless needs
  absolute paths.
- **Gen 6 text decoder** (`xy-script/src/textfile.rs`, pk3DS `TextFile.cs`
  port): per-line XOR crypt, variable records, XY variable table, char
  remaps. Empirical language maps (bank 080 decode probe): gametext
  `a/0/7/2..9` = jpn, jpn2, eng, fre, ita, ger, spa, kor; storytext
  `a/0/8/0..7` = jpn-kana, jpn-kanji, eng, fre, ita, ger, spa, kor.
  4,600 banks → JSON under `src_extracted/text/`. Golden pins: ENG species
  line 1 = "Bulbasaur", line 658 = "Greninja"; JPN = フシギダネ; item 0 is
  the "???" placeholder (my first expectation was wrong — decoder was
  right).
- **BCRES v1 inspector** (`xy-gfx/src/bcres.rs`): header validation, 4CC
  chunk census (DICT/CFLT/CALT/CENV/CCAM/CANM…), NUL-terminated DICT name
  harvesting (u16-prefix assumption was wrong — BCH≠BCRES there). Full
  mesh/texture decode stays Phase 2.
- **CROs blocked**: AES-CTR per-file keystreams, no `CRO1` magic, no
  offline key material (no boot9/console). Documented in AGENTS.md/MAP.md
  with the unblock path (console-dumped decrypted CROs → CRO1 parser →
  per-module Ghidra import).
- `Data/src_extracted/MAP.md` written as the master output index.
- Gates at merge: `cargo test` 38/38 · clippy `-D warnings` clean · fmt
  clean.

### 2026-10-05 (latest) — GARC unlocked: parser + dsdecmp LZ-0x11 ✅

Phase 1 core landed in `xy-archive`:

- `garc.rs` — v4 GARC parser, layout documented from hexdump recon
  (`a/0/0/1`, `btl_ai.garc`) against pk3DS `GARC.cs`. Validates fileSize,
  FATO↔FATB offset consistency, FATB size arithmetic, FIMB bounds, and
  every subentry range before returning anything. `subfile(entry, bit_id)`
  borrows by **vector bit id**, not position (sparse vectors are real:
  `a/0/2/0` entry 0 uses bits 0 and 7).
- `lzss.rs` — LZ-0x11 payload decoder, faithful port of pk3DS `LZSS.cs`
  (dsdecmp). **The indicator mapping is the dsdecmp one**, not Nintendo's:
  S≥2 2-byte, S==0 3-byte +0x11, S==1 4-byte +0x111. Empirical pins: ~4,600
  real subfiles decode to their exact declared size with coherent content
  (`menu_app_active.bclan` et al.); the corpus sweep (255 GARCs under
  `romfs/a/`) found zero parse/decode failures. `try_decompress` mirrors
  pk3DS's try/catch: first byte `0x11` alone proves nothing
  (`btl_ai.garc` file 3 is raw despite starting `0x11`).
- CLI: `xy archive list`, `xy archive unpack -i <garc> -o <dir>
  [--decompress]`, pk3DS-compatible output names.
- Golden: `crates/xy-archive/tests/garc_golden.rs` (container self-checks,
  CGFX embedded-size == FATB length, pinned decoded BCLAN bytes, raw
  false-positive). All skip-if-absent.
- Kuriimu2 GARC4 refuses XY's v4 files → parity rests on pk3DS semantics +
  internal cross-checks; full independent byte-parity still open.
- Gotcha for future LZ work: `xy-core::lz10`'s LZ11 branch (Nintendo
  mapping) has never been validated against real data — real manual pages
  are all LZ10. Fix or remove it when a genuine Nintendo-LZ11 asset shows
  up; do **not** use it for GARC payloads.

### 2026-10-05 (later) — workspace restructured to the six-crate blueprint ✅

Adopted the project blueprint's domain-crate layout (user decision). The
validated parsers moved **unchanged** (import paths only), so parity
provenance carries over:

- `xy-decomp` split into `xy-core` (error/util/lz10/sniff), `xy-archive`
  (romfs/bcma), `xy-audio` (bcwav + new `wav.rs` for the RIFF writer),
  `xy-gfx` (smdh + a thin png-aware error wrapping `xy_core::Error` — keeps
  the `png` dep out of core per the dependency rule).
- New documented stubs: `xy-script` (msg.garc / event scripts),
  `xy-battle` (stats / encounters / trainers) — both blocked on GARC.
- `xytools` binary replaced by the root package `xy`; commands are now
  domain-nested (`xy archive romfs|darc|manual`, `xy audio bcwav`,
  `xy gfx icon`, `xy core lz`, top-level `xy info`).
- Error enum slimmed in core (no `Png` variant); WAV writer moved to
  `xy-audio`; `sniff()` now recognizes the 0x100-prefixed NCSD of real
  `.3ds` files and the `CRAR` GARC magic.
- **Post-restructure validation (all re-run):** `cargo test` 15/15 · clippy
  `-D warnings` clean · fmt clean · manual pipeline byte-identical vs
  `Data/extracted_assets/manual` (3663 assets, `diff -r` empty) · BCWAV
  byte-identical vs reference WAV · SMDH icon pixel-identical vs reference
  PNG (PNG container bytes differ — encoder settings, pixels match).
- Doc fixups: X/Y title IDs un-swapped in PLAN.md Phase 4 and CLI/test doc
  comments (SMDH confirms `0055D00` = Pokémon X = what we have).

### 2026-10-05 — bcwav DSP-ADPCM port merged and validated ✅

Session work folded into the workspace (superseded the orphaned single-crate
scaffold at the repo root, now deleted):

- `bcwav.rs` rewritten to auto-detect both CWAV variants:
  - reference-table (standard) — PCM8/PCM16 as before;
  - **chunked bannertool/GYTB layout** (what XY's banner actually uses:
    absolute `{flags, off, len}` chunk table at 0x14, sequential
    non-interleaved channels). The previous reference-table reader would
    reject these files.
- **DSP-ADPCM (codec 2) implemented** — port of vgmstream
  `ngc_dsp_decoder.c`, i64 intermediates (extreme coefs overflow i32).
  Both real banner CWAVs decode: `banner_jingle.bcwav` and
  `banner/banner.bcwav` (identical streams: 32728 Hz, 2 ch, 95372 samples).
- `xytools bcwav` now converts DSP-ADPCM → WAV (CLI help updated; also fixed
  swapped X/Y title IDs in the crate doc comments).
- New `crates/xy-decomp/tests/bcwav_golden.rs`: metadata + hand-verified
  frame-0 samples + **full byte-parity** against the reference WAV in
  `Data/extracted_assets/audio/`. All skip-if-absent.

Validation gates at merge time: `cargo test --workspace` 15/15 · clippy
`-D warnings` clean · fmt clean · CLI output byte-identical to the Python
reference decoder on both banner files.

Gotcha recorded in AGENTS.md (and worth keeping in mind for GARC work):
in the chunked layout, DSP coefficients start at
`INFO+0x20 + ch*8 + ch*0x14` — dropping the `ch*0x14` channel-block stride
reads block headers as coefficients and yields plausible-but-wrong audio that
still passes weak "not silence" checks. Golden tests pin exact samples, not
just peak levels.

Known gaps left open (feeds Phase 0/1):

- Reference-table-variant DSP coefficients are still unmapped (`Unsupported`)
  — needs a real DSP sample of that variant to validate against.
- `Data/extracted_assets/banner/` also holds `banner.cbmd` + 17 `.bcmdl`
  animation models (sniffed via `xytools info`) — CBMD/BCMDL parsing not
  planned anywhere yet; parked until after GARC.

### 2026-10-06 — Suite layer: xy-curate library + xy-convert backend + headless CLI commands ✅

- **`xy-curate` library extraction.** `src/curate.rs` (+`gfx_convert`,
  `csharp`) moved verbatim into `crates/xy-curate` so the pipeline is a
  reusable library. New `build_with_progress` (per-stage/file progress
  callback + `AtomicBool` cancellation) and `manifest.rs`
  (`read_manifest`/`read_validation`) for provenance browsing. The `xy
  curate` CLI now calls the library; behavior is unchanged.
- **`xy-convert` crate** — the inspection/conversion backend:
  - `detect_kind` → `FileKind` routing (magic + GF-container probing,
    flags for models/textures inside BCH/CGFX/PC payloads);
  - `model::SceneModel` — one UI-neutral decoded model (meshes with
    positions/normals/UVs/skinning, materials, bones, decoded RGBA
    textures, animations metadata) from BCH, CGFX, or any PC/PF/PT
    nesting; `collect_decoded_textures` mirrors `xy gfx texture`;
  - `convert` — one-shot conversions returning in-memory `OutputFile`s:
    `models_to_glb`, `textures_to_png`, `bcwav_to_wav`,
    `text_bank_to_json`, `write_outputs`;
  - `inspect` — kind/magic/size/CRC32/FNV-1a/SHA-256 file info;
  - `media` — ffprobe JSON parsing (serde-free scanner) + ffmpeg
    decode-to-WAV / MP4 convert / frame extraction;
  - `render` — headless z-buffered software rasterizer over
    `SceneModel`: textured/material/wireframe/normals/skeleton modes,
    orbit camera with auto-fit, perspective-correct UVs, nearest
    sampling, single shot or turntable → PNG. Unit-tested
    (coverage/z-change/joints/turntable).
- **GLB export** (`xy-gfx/src/gltf.rs`): binary glTF 2.0 with embedded
  PNG textures, PBR-lite materials (diffuse/emission, BLEND alpha for
  alpha formats), skin + inverse-bind matrices. Neutral `GltfScene`
  input with `from_bch`/`from_cgfx` mappers. The matrix-roundtrip unit
  test caught a cofactor-inverse index bug → replaced with
  Gauss-Jordan (validated by roundtrip test). CLI: `xy convert glb`.
- **New CLI surface** (all headless): `xy inspect [--hashes]` (kind +
  magic + hashes + model/texture census), `xy search name|kind` (walk a
  root by substring or detected format), `xy media info|frames|mp4`,
  `xy manifest list|lookup|validate` (manifest.tsv browsing, raw↔curated
  provenance both directions, on-disk re-verification), `xy gfx render`
  (headless viewer replacement: `--mode/--yaw/--pitch/--width/--height/
  --cull/--turntable/--model`), richer `xy info`.
- **GUI decision:** a iced-based GUI (`ui/` package) was prototyped this
  session and then **removed the same day** — the suite stays a pure
  headless CLI; the `xy-convert`/`xy-curate` APIs are the stable seam if
  a front-end is ever wanted again. Rendering/preview capabilities live
  on as `xy gfx render` / `xy media frames`.
- **Data note:** the ROM dumps under `Data/` were moved out of the
  workspace this session (only `src_extracted/decomp/dllbattle_c`
  remains) — the new CLI commands were validated with synthetic media
  (ffmpeg testsrc/sine) and synthetic curated trees instead; re-run the
  goldens once the dumps return (they skip cleanly when assets are
  absent).

### 2026-10-06 — GLB hardening: validation gate, TRS rigs, textures, CANM export ✅

- **User-reported**: Blender said "not a valid Blender file" on our GLB.
  Root cause: File→**Open** (native .blend) instead of File→**Import →
  glTF 2.0**. Response: make validity provable, not asserted.
- **`glb_check` validator** (`crates/xy-convert/src/glb_check.rs`, `gltf`
  crate, serde-free consumers unaffected): schema parse + every
  accessor decoded against the BIN chunk, image views bounds-checked,
  skin joint/IBM counts cross-checked, POSITION min/max enforced,
  animation sampler inputs decoded. Wired into `xy convert glb
  --validate` (exits 1 on any error) and `xy inspect *.glb` (census).
- **Exporter upgrades** (`xy-gfx/src/gltf.rs`):
  - bones emit as **TRS nodes** (translation/rotation-quat/scale) instead
    of matrices — animatable rigs; quat-from-euler verified against the
    matrix path by a parity test (which caught a broken test-helper
    formula, then passed);
  - `xy convert glb --textures <pack>` merges a family texture container
    into the scenes before export (model PCs carry none);
  - **CANM skeletal-animation export** (times = frames/30, linear/step;
    euler→quat per key) via `GltfAnimation`/`GltfAnimChannel`, built both
    in `from_cgfx` and the xy-convert SceneModel path; round-trip test
    exports an animated scene and re-validates it with `glb_check`.
  - duplicate model names in one container get `_2`, `_3`… suffixes
    (4× TestPlane in 0001.bin used to overwrite).
- **Verified end-to-end on real data**: re-exported pm0001_00 with the
  0006 PT pack → 12 embedded textures, 5 textured materials, 71-joint
  TRS skin — `--validate` VALID; Blender headless import: 8 meshes /
  3,541 verts / 1 armature (71 bones) / 5 textured materials / 4
  images; proof render saved next to the exports.
- **Census finding**: zero `CANM` chunks across all of `a/0/0/7` and
  `a/0/3/3` (only TXOB/MTOB/CNOD) — XY CGFX genuinely carry no
  skeletal animations; motions remain the v2 GFMotion project. The
  animation path is validated synthetically and ready for ORAS-style
  files.
- Gates: clippy `-D warnings` clean, 27 test suites green.
