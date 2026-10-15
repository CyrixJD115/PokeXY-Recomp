# docs/decomp/ — matching-decompilation track

The "proper source" route: Ghidra automation (PyGhidra under Ghidra 12 —
setup in [ghidra-scripts.md](ghidra-scripts.md), this folder), symbol CSV
export, the 3DS-Decomp-Pipeline split → compile → byte-compare → relink
loop, and objdiff.

**In scope**
- Pipeline usage and gotchas: what breaks, which patches were needed, which
  compiler/flags match.
- Per-module decomp progress beyond the headline numbers (which live in
  [../general/findings.md](../general/findings.md)).
- Symbol workflow: Ghidra CSV export, static.crs overlay, naming decisions.
- Relink status (134/134 byte-identical today) and how it was proven.

**Out of scope:** per-module *content* findings (→ `../dll/`); the static
recomp track (→ `../recomp/`).

**Conventions:** kebab-case topical files; newest-first dated entries for
running logs.

## Files

- [ghidra-scripts.md](ghidra-scripts.md) — running the 3DS Ghidra script
  suites under Ghidra 12 via PyGhidra (Jython-removal workaround + patches)
- [armcc-toolchain.md](armcc-toolchain.md) — the retail toolchain is ARM
  Compiler 5 (armcc), not GCC; setup, fingerprinted flags, 38 functions
  masked-matched across `DllDialogCommon`, `DllNumberInput`,
  `DllStrInput`, `DllFatalErrorProc`, `DllLangSelect`, `DllConfig`,
  `DllHeading`, `DllCommWait`, `DllSalon`, `DllCommendation`,
  `DllBattleHouseConnect`, `DllGameClearSave`, `DllTheEnd`, `DllUS3DDun`,
  `DllTrainingAppManager`, `DllUSDynamo` and `DllUSBadgeCheckGate`, open residue-class puzzle + two open block-layout/
  pointer-materialization puzzles (`0xd20`, `0x710`) + an open
  jump-table specimen (`0xdd8`) + an open store-merge puzzle (`0x31b8`).
  Verifier + matched specimens: `xy-decomp/armcc-matching/`.
