# docs/dll/ — the game's dynamic code layer

The 133 plaintext CRO modules + `static.crs` + the code.bin main binary:
GameFreak's DLL graph. One file per module, named exactly like the module
(`DllBattle.md`), plus cross-module notes (import graph, load order,
subsystem clustering).

**In scope**
- Per-module discoveries: real symbol exports/imports, subsystem behavior
  (battle engine in DllBattle, bag UI in DllBag, …), decomp/recomp status.
- Symbolication wins via `static.crs` (461 named exports; match code.bin
  addresses on `addr & !1` — the Thumb bit).
- The cross-module import graph: 78 import modules in the static module =
  every game subsystem by name.

**Out of scope:** the CRO0 container spec (→ `../3ds/` and the
`xy-archive/src/cro.rs` module docs); decomp *method/tooling*
(→ `../decomp/`).

**Conventions:** module files use the exact module name; findings inside are
newest-first dated entries.

## Files

— none yet.
