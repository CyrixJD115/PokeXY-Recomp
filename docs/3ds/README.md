# docs/3ds/ — CTR platform notes (game-agnostic)

Knowledge about the 3DS platform itself: hardware, OS, system formats, and
tooling. If a note would still be true for a different 3DS game, it belongs
here; if it is about Pokémon X's data or code, it belongs in `../formats/`,
`../knowledge/`, or `../dll/`.

**In scope**
- ARM11/Thumb specifics, svc/service interfaces, TLS layout, linking model.
- System containers *as formats*: CCI/NCCH/IVFC/RomFS/CXI, CRO0, SMDH,
  update-title (CIA) mechanics.
- Crypto and key-material mechanics (the key material itself never leaves
  `.Data/Rom/nand/` — never commit or quote it).
- Platform tooling quirks: ctrtool, makerom, 3dstool, Ghidra 3DS suites,
  devkitARM, Kuriimu2 (and its `/mnt` path mangling).

**Out of scope:** game asset formats (→ `../formats/`), the game's own CRO
modules (→ `../dll/`), game data organization (→ `../knowledge/`).

**Conventions:** kebab-case topical files (`ncch-ivfc.md`); newest-first
dated entries for log-style notes; cite how each claim was verified. The
CRO0 container spec itself lives in the `xy-archive/src/cro.rs` module docs —
notes here cover the platform context around it.

## Files

— none yet.
