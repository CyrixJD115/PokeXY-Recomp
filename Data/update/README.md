# update/ — v1.5 card-update dump slot (empty until you dump it)

Pokémon X/Y shipped on v1.0 and received a v1.5 card update. The project is
currently pinned to **v1.0**; landing the update here unlocks the v1.5
code.bin (and its fixed bugs) for the recomp + matching-decomp tracks.

## What to put here

Dump the update titles from a console with the update installed
(3dsutils/godmode9 "dump title"), decrypted, and drop them in this folder:

| Game | Update title ID | Expected update code.bin |
|---|---|---|
| Pokémon X | `0004000e00055d00` | **5,169,152 bytes** |
| Pokémon Y | `0004000e00055e00` | same layout |

Sanity checks on arrival (mirrors `xy`-tooling pins for v1.0):

- update `code.bin` decompressed size is exactly **5,169,152** bytes
  (v1.0 base is 5,156,864 — if you see that number you grabbed the wrong
  partition);
- 133 CRO modules present (same count as base), `static.crs` alongside;
- keep dumps/extracts in clearly-named subfolders, e.g. `00055d00_v1.5/`.

## What changes when it lands

- Re-run `3dsrecomp analyze/build/verify` against the update code — the
  95.7% / 0-mismatch results were measured on v1.0 and do **not** carry
  over automatically.
- Ghidra project: a second program set (or a fresh project) — v1.5
  addresses shift; symbol CSV exports must be re-exported per version.
- Matching-decomp pipeline: new `orig/` set; splits regenerate.
- Version-pinning table in `docs/findings.md` gets a v1.5 column.

Until then, every tool here assumes v1.0 inputs.
