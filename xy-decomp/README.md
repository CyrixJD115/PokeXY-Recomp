# xy-decomp — matching decompilation track

Rebuilds the original game code **byte-for-byte**, so every future
hand-written C file has a hard parity gate. The loop:

1. **Corpus** — `ghidra/` (this dir) holds the Ghidra 12/PyGhidra suite:
   the labeling chain (`Label3dsSVCs` → `Label3dsHandles` →
   `Label3dsCTRFns` → `Set3dsTlsType`) driven by `xy_ghidra_3ds.py`, and
   `DecompileAll.java` (exports every function to C; force-creates
   `module_entry` for zero-function stub modules). Output:
   `.Data/Source_Extracted/decomp/` — 38,385 C functions (14,864 code.bin +
   23,521 across the 133 CRO modules).
2. **Symbols** — `ghidra/ExportSymbolsHeadless.py` exports per-binary
   symbol CSVs (`Location,Name,Namespace,Mode,Size,Segment`) in
   3DS-Decomp-Pipeline format into `.Data/Source_Extracted/pipeline/symbols/`
   (code.bin: 16,545 symbols; CRO CSVs shifted −0x100000 into the
   pipeline's text-offset frame; duplicate `thunk_FUN_*` names deduped).
3. **Split → relink → diff** — 3DS-Decomp-Pipeline (`.thirdparty/`)
   generates `split/` and relinks with arm-none-eabi; **all 134 binaries
   (code.bin + 133 CROs) recreate SHA-256-identical originals** from their
   72,742 split objects. `objdiff.json` (134 units) drives
   objdiff (`.thirdparty/`) for per-unit diffs.

Gotchas: `--use-splits-only --objdiff` hits an upstream UnboundLocalError —
use plain `--objdiff`; `getImageBase()` returns 0 for raw-binary imports
(use `getMinAddress()`). Setup + patches: `docs/ghidra-scripts.md`;
metrics: `docs/findings.md`.

Workspace (game-derived, machine-local): `.Data/Source_Extracted/pipeline/` —
`orig/`, `symbols/`, `split/`, `out/`, `objdiff.json`. See
`.Data/Source_Extracted/MAP.md` for the full index.
