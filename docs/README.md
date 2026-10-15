# docs/ — the research log for the Pokémon X (and Y) de/recomp

This tree is where the project's knowledge lives. The code verifies; the docs
remember. Primary target: **Pokémon X** (`kujira-1`, title `...0055D00`);
Pokémon Y (`kujira-2`) shares every format and is held to the same parity
sweeps.

The operating rule is the **Documentation duty** in
[AGENTS.md](../AGENTS.md): every durable finding — format quirk, working
command, tool gotcha, mapping rule, measurement, dead end, disproven theory —
is written into the matching folder below *in the session that found it*.
AGENTS.md stays the condensed layer; this tree holds the detail. This README
is the map: keep it current whenever a file or folder is added.

## Folder map

| Folder | Charter |
|---|---|
| [3ds/](3ds/) | CTR platform knowledge, game-agnostic: hardware/OS (ARM11/Thumb, services), system containers (CCI/NCCH/IVFC/RomFS/CXI, CRO0 *as a format*, SMDH), title/update/crypto mechanics, 3DS tooling quirks |
| [formats/](formats/) | Game data formats & open format RE: GARC v4, LZ-0x11, BCH, CGFX + rev-9 CNOD, PC/PF/PT, GFMotion v2, BCMA, text banks — recon narrative, cross-tool parity, open problems |
| [knowledge/](knowledge/) | Game-domain knowledge: how XY organizes its world — romfs data map, `a/0/0/7` families, pm-id↔dex mapping, `a/0/0/6` form table, text bank indices, battle data (`a/2/1`), AMX scripts |
| [dll/](dll/) | The game's dynamic code layer: 133 CRO modules + `static.crs` + code.bin — per-module exports/imports, subsystem behavior, decomp/recomp status |
| [decomp/](decomp/) | Matching-decompilation track: Ghidra automation, symbol CSV export, 3DS-Decomp-Pipeline, objdiff, relink status |
| [recomp/](recomp/) | Static-recomp track: 3dsrecomp/Zakuro usage, port build + overrides, per-function verification harness |
| [runtime/](runtime/) | Native runtime track: Zakuro fork vs full-native decisions, asset bridge, runtime-vs-hardware parity |
| [general/](general/) | Project-level: decisions + rationale, workflows, environment quirks, cross-track meta |
| [assets/](assets/) | Generated images embedded in the root README (progress bar, central treemap) — regenerate with `python3 xy-heatmap/heatmap.py readme` (renderer: `xy-viz`, charming/ECharts) |

Each folder's own `README.md` holds its detailed charter and file index —
the summaries above are one-line pointers. If a note has an obvious
single-topic home, it goes there; `general/` is the fallback, not the default.

## Migration note (2026-10-11)

The four original root-level docs were sorted into the folders:
`findings.md` → [general/](general/) (it spans recomp + decomp + AMX
measurements), `ghidra-scripts.md` → [decomp/](decomp/),
`runtime-architecture.md` → [runtime/](runtime/), and
`PokemonX_Decomport_Report.md` → [general/](general/). Nothing else moved;
stale root links in old notes should be re-resolved through the folder
indexes above.

## Writing conventions

- One topic per file, `kebab-case.md` (`gfmotion-v2.md`); module files may
  use the exact module name (`DllBattle.md`).
- Log-style files take newest-first dated entries: `## 2026-10-11 — title`.
- Every claim states how it was verified (command, golden test, tool +
  version) or is explicitly marked *inferred* / *unverified*.
- Dead ends are findings: record what was tried and why it is wrong, marked
  "wrong, kept for history" — the repo culture already does this and it has
  paid for itself (e.g. the CRO "encryption" theory).
- Link evidence instead of restating: `path:line` into this repo, the parser
  module doc that owns the offset table, or the upstream tool.
- Legality: structures, offsets, and measurements are facts — fine to write
  down. Never paste game assets or wholesale game-derived code; `.Data/`
  content stays machine-local.
- Adding a file or folder ⇒ update this README and the folder's README index
  in the same change (Documentation duty).

## What to explore — the open frontiers

Ranked roughly by unlock value. Each names the folder where the resulting
notes belong; the pointers are starting points, not exhaustive briefs.

1. **CNOD mesh geometry** → `formats/` — XY's rev-9 CGFX world/effect models
   keep their geometry in `TypeId 0x40000001` CNOD chunks (nested
   self-relative descriptor tables, offset triples 0x980/0xa80/0xb40…). No
   public tool decodes them; SPICA throws `EndOfStream`. Start: the parked
   notes in `xy-gfx/src/cgfx.rs` over `a/0/3/3` world models.
2. **GFMotion v2 skeletal motions** → `formats/` — the real animations live
   in `a/0/9/3` (945 files) + `a/0/9/4` (579) as v2 bitstreams (magic
   `02 00 09 00`, quantized, name-less). SPICA parses only the ORAS v6
   format (`0x00060000`). The PF containers' BCHs hold only per-form
   material/visibility anims — the skeletal curves are in the v2 stream.
3. **`a/0/0/6` form-group table** → `knowledge/` — a u16 table of form-group
   ids whose species→group indexing has unresolved skips beyond species 3.
   Related bug: `xy curate` pokemon naming must trust the BCH's own
   `pm####_form` name, never this table (the old "8,120 slots" reading was
   wrong and mislabeled `0001.bin` as Bulbasaur).
4. **CRO module census** → `dll/` — DllBattle is imported + decompiled; the
   other 132 modules are queued through the same Ghidra pipeline.
   `static.crs`'s 461 named exports symbolicate the code.bin decompilation
   (match on `addr & !1` — Thumb bit). One file per module as it falls.
5. **Per-function native verification** → `recomp/` — recomp covers 69,038
   functions (95 %) but **zero** are behaviorally verified; wiring the
   3dsrecomp verify / objdiff per-unit harness is the credibility gap.
   Separately, matching-decomp's compiler-parity blocker is now identified
   (armcc 5, not GCC) and 358 functions across 113 modules are masked-matched
   proof-of-concept — see [decomp/armcc-toolchain.md](decomp/armcc-toolchain.md).
   Read "modules" as breadth, not depth: only 41 of those 113 have more
   than one matched function (see the caveat at the top of
   `xy-decomp/armcc-matching/README.md`), so most of each module's function
   count is still untouched. Scaling to the remaining ~38,347 functions is
   the follow-on work.
6. **Independent GARC v4 parity** → `formats/` — current validation is
   self-describing cross-checks + the 255-GARC corpus sweep; Kuriimu2
   refuses XY's v4. Any independent implementation found is worth a
   byte-parity run.
7. **`xy-core` LZ11 branch** → `formats/` — unvalidated; LZ-0x11 payloads
   must keep going through `xy-archive/src/lzss.rs` (dsdecmp indicator
   mapping, *not* the core mapping). Validating the core branch is a small,
   self-contained task.
8. **Battle data structures** → `knowledge/` — `xy-battle` is a stub; the
   data is reachable at `a/2/1/*`. Map the stats / encounters / trainers
   GARCs (Project Pokémon's Gen 6 docs are the reference).
9. **AMX scripts** → `knowledge/` — per the founding report, a large share
   of game logic (events, overworld, battle glue) lives in AMX scripts and
   XAMX dis/assembles them. Locate the payloads in XY's romfs and scope
   what they govern.
10. **Zakuro runtime track** → `runtime/` — execute the two-track strategy
    from [runtime/runtime-architecture.md](runtime/runtime-architecture.md):
    Zakuro fork + asset bridge vs full native runtime.
11. **Y parity sweep** → `general/` — formats are shared by design; run the
    census/parity sweeps against the Y dump to prove nothing is X-specific
    in the parsers.
