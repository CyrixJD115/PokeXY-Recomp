# Findings — measured results on real Pokémon X/Y dumps

All numbers below were measured on the maintainer's machine (12-core Linux,
GCC 16, Rust 1.97) against **v1.0 base-cart dumps**. Reproduce with the same
toolchain roots: [fearkov/3dsrecomp](https://github.com/fearkov/3dsrecomp),
[fearkov/zakuro](https://github.com/fearkov/zakuro),
[3DS-Decomp-Pipeline](https://github.com/AlgebraManiacABC/3DS-Decomp-Pipeline).

## Version pinning (read this first)

`code.bin` differs between game revisions — everything (recomp, decomp,
patches) must target one version:

| Revision | decompressed `code.bin` size |
|---|---|
| X v1.0 (base cart, what we use) | **5,156,864 bytes** (SHA-256 `0f6d3e1d…`) |
| X v1.5 (update title `0004000e/00055d00`) | 5,169,152 bytes |

The v1.5 binary is 12,288 bytes larger; mixing base code with update data
crashes or silently misbehaves. Extract v1.5 from the update title with
GodMode9 (`0004000e/00055d00` → exefs → `.code`, LZSS-decompressed) if you
want it; just don't mix versions across tools.

## Static recompilation (3dsrecomp)

`3dsrecomp analyze PokemonX.3ds`:

| Metric | executable | CRO modules | total |
|---|---|---|---|
| Functions discovered | 26,493 | 44,670 | **71,163** |
| Code reached | 96.2 % | 95.3 % | **95.7 %** |

Residual gaps: 673 dead-end bytes across the executable (largest single run
952 B) plus a 2 KB unreached run in `DllTitle` — all covered by Zakuro's
interpreter at runtime.

`3dsrecomp build` → 664 files, **679 MiB of generated C**, 529/529 compile
units, **0 hand-written overrides needed**; library links to a 296 MB x86-64
`.so` (75,452 `.symtab` symbols). ~39 min cold on 12 cores.

`3dsrecomp verify` (built with `--features verify`) re-executes recompiled
functions against Zakuro's interpreter and compares state:
**24,063 functions checked, 0 mismatched** (84 s).

The standalone `port` binary is the same generated C linked with the Zakuro
runtime instead of being loaded as a library.

## Matching-decomp verification loop (3DS-Decomp-Pipeline)

With Ghidra-exported symbol CSVs (16,545 symbols for `code.bin` alone),
the pipeline splits every original binary into per-symbol ELF objects:

- 134 binaries → **72,742 split objects**
- Relinking splits only and recreating the originals reproduces
  **134/134 binaries SHA-256-identical** — the pipeline's split/link/recreate
  machinery is exact on these games.
- `objdiff.json` (134 units) tracks per-function matching progress as
  decompiled sources get added. Compiler parity (matching GCC version/flags)
  is the next milestone; without it compiled-vs-original matching stays at 0 %.

Gotchas hit along the way (relevant to any XY/Y setup):

- Ghidra CRO imports are conventionally based at the text file offset
  (0x180-ish). If your Ghidra project bases CROs at 0x100000 like ours,
  exported symbol addresses need a −0x100000 shift before the pipeline.
- Ghidra names every thunk to the same target identically
  (`thunk_FUN_…`) → duplicate ELF symbols at link time. Deduplicate by
  suffixing `_` + address (the exporter's `unique` mode does this).

## AMX script layer (XAMX)

XY's event/overworld logic largely lives in AMX bytecode, not ARM code —
editable without touching the recompilation at all:

- global scripts `a/0/3/1`: **59/59 disassembled**
- map scripts `a/0/1/2`, plus `a/0/9/0`, `a/1/0/7`, `btl_ai.garc`
- disassemble→reassemble round-trip is **byte-identical** to the originals
  (tool: [BlastSlimey/XAMX](https://github.com/BlastSlimey/XAMX); it prompts
  interactively, so feed it stdin when scripting)

## Build time cheat-sheet (12-core, cold cache)

| Step | Time |
|---|---|
| 3dsrecomp `build`/`port` C compile | ~39 min |
| port cargo link | ~5-10 min |
| `verify` | ~1.5 min |
| same C compile with warm ccache | minutes |
