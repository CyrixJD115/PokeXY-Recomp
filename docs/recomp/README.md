# docs/recomp/ — static-recompilation track

The fast path to native: 3dsrecomp analyze/build, the generated standalone
port (`xy-recomp/build.sh` + `overrides/`), Zakuro runtime integration, and
the per-function verification harness.

**In scope**
- Tool versions, flags, and invocations that work (3dsrecomp + Zakuro are
  moving targets under `.thirdparty/` — pin what worked).
- Port build gotchas: C override decisions, function coverage gaps,
  interpreter-fallback cases.
- The verification story: recomp covers 69,038 functions (95 %) but 0 are
  behaviorally verified — harness design and results land here.

**Out of scope:** runtime architecture strategy (→ `../runtime/`); the
matching-decomp track (→ `../decomp/`); headline metrics (→
[../general/findings.md](../general/findings.md)).

**Conventions:** kebab-case topical files; newest-first dated entries for
running logs.

## Files

— none yet.
