# docs/knowledge/ — game-domain knowledge

How Pokémon X organizes its world: data maps, id mappings, bank indices,
script systems — knowledge about the *game*, as opposed to byte layouts
(→ `../formats/`), platform mechanics (→ `../3ds/`), or its code modules
(→ `../dll/`).

**In scope**
- The romfs data map: what lives at `a/*/*/*`, which GARC is which system.
- The `a/0/0/7` 8-file model families and the `a/0/0/6` form-group table
  (species→group indexing still unresolved beyond species 3).
- pm-id ↔ national-dex mapping (the Kalos quirk: ids ≥650 are XY-internal,
  no uniform dex mapping; render-verified bounds).
- Text bank index map (`a/0/7` gametext, `a/0/8` storytext, bank numbers
  per pk3DS `TextReference`).
- Battle data (`a/2/1/*`): stats / encounters / trainers.
- AMX script coverage: where the scripts live, what they govern.

**Conventions:** kebab-case topical files (`pm-id-mapping.md`); state how
each mapping was verified (render check, bank decode, cross-reference).

## Files

— none yet.
