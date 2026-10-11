# C overrides

Hand-written C replacements that `3dsrecomp port/build` links in place of
recompiled functions. Each override targets one function by its generated
name (`f_<vaddr>` for code.bin, `m<XXX>_f_<vaddr>` for CRO modules —
module-relative, 0x180-based) and receives the `Context *` ABI from
`recomp.h`.

Workflow:

1. Write `my_override.c` implementing `void f_00123456(Context *ctx)`.
2. Declare the target in an overrides file passed as
   `--overrides <file or dir>` to `3dsrecomp port` (see
   `.thirdparty/3dsrecomp/docs/overrides.md` for the exact format).
3. Rebuild with `xy-recomp/build.sh`.

Nothing here yet — the first candidates are whatever the Zakuro interpreter
log flags as hot or broken (interpreter hits feed 3dsrecomp hints files).
