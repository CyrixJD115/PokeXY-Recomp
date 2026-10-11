# docs/formats/ — game data formats & open format RE

How Pokémon X/Y bytes are laid out: container and asset format specs, decoder
parity notes, and the open reverse-engineering problems.

**Relationship to code:** the authoritative offset tables live in the parser
module docs (`xy-extraction/*/src/<format>.rs`), kept in sync with the code.
Notes here hold the investigation *around* them — recon, dead ends,
cross-tool parity, open questions — and link both ways when both exist.

**In scope:** GARC v4 + LZ-0x11, BCH (CtrH3D), CGFX + rev-9 CNOD, PC/PF/PT
containers, GFMotion v2, BCMA/DARC, BCWAV, Gen-6 text banks, SMDH decoding,
glTF/GLB export decisions.

**Open problems owned here** (see the frontier list in
[../README.md](../README.md)): CNOD geometry, GFMotion v2, independent GARC
parity, the unvalidated `xy-core` LZ11 branch.

**Conventions:** one format per file, kebab-case (`gfmotion-v2.md`);
newest-first dated entries when the file is a running log.

## Files

— none yet.
