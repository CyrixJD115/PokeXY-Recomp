Module: DllFieldEventSlopeSlip
Offset: 0x268
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A thirty-eighth module (new, not yet in the README table). Sits right
after the import-veneer table (0x180-0x227) and a real init/notification
function (0x228-0x267); another real function follows immediately at 0x270.
Same recurring tail-of-veneer-table leaf-stub shape as every other module
mined so far. Starting candidate was `DllDebugPokeMake`, whose only
function (the whole 0xac-byte `.text` is one `nnroControlObject_` 5-case
property get/set switch) reproduces the exact jump-table + 3-literal-pool
/nop-interleave default-case residue already flagged as unresolved for
`DllDialogCommon@0xdd8`; that same byte-identical 0xac function also
appears verbatim in `DllDebugProcLoop` (cmp'd identical). Left both
untried (not logged to attempted.md myself, just noting it here) and moved
to a tractable module instead. `DllFieldEventSlopeSlip`'s own tail also has
the identical switch function at 0x4a8 (same literal-pool shape) and a
call-through-then-tail-branch wrapper at 0x468-0x47f plus a no-arg
init-globals function at 0x47c-0x49c (writes -1,-1 then 0,0 to two
literal-pool global addresses) — both look like promising next easy wins
for a future agent.
