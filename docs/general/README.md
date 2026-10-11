# docs/general/ — project-level notes

Everything that spans tracks or is about the project rather than the game:
decisions + rationale, workflows, environment quirks, cross-cutting tooling
notes.

This is the fallback folder, not the default. If a note has an obvious
single-topic home — `3ds/`, `formats/`, `knowledge/`, `dll/`, `decomp/`,
`recomp/`, `runtime/` — it goes there instead. When a file here grows a
clear theme, move it to the right folder and leave a pointer.

**Conventions:** kebab-case topical files; newest-first dated entries for
running logs.

## Files

- [findings.md](findings.md) — measured recomp/decomp/AMX results on real
  dumps + version pinning (X v1.0 = 5,156,864 B `code.bin` — never mix with
  v1.5); this is the run-metrics log the Documentation duty names
- [PokemonX_Decomport_Report.md](PokemonX_Decomport_Report.md) — the Oct 10
  2026 landscape survey that founded the project (external tooling, links,
  legality framing)
