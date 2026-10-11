# xy-runtime — native runtime track

Home for the runtime layer the recompiled code runs against: HLE OS
services, scheduler/threads, GPU/FS/audio shims. Seeded from a working
clone of [fearkov/zakuro](https://github.com/fearkov/zakuro) — the HLE 3DS
runtime that executes 3dsrecomp'd code natively (`zakuro-cpu/gpu/fs`
crates).

- `zakuro/` — working checkout (machine-local, own `.git`). The pristine
  upstream copy that `xy-recomp/build.sh` passes as `--zakuro` still lives
  in `.thirdparty/zakuro`; point the builder at this checkout when runtime
  work starts.
- Everything here is upstream code or game-derived output — only this
  README is committed.

Related: `xy-recomp/` (produces the recompiled code + standalone port),
`.Data/src_extracted/recomp/` (analyze/build output), `.Data/ports/`
(generated port trees).
