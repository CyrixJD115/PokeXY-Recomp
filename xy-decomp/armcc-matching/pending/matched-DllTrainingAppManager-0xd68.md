Module: DllTrainingAppManager
Offset: 0xd68
Size: 0x8
Match: 2/2
One-line: `xy_system::bg::System::SetIsCallDraw(bool)` — trivial byte-field setter (`this->is_call_draw = v;`), void return.
Note: A thirteenth module. The module's own named export table gives the
real mangled symbol (`_ZN9xy_system2bg6System13SetIsCallDrawEb`), confirming
what DllStrInput@0x666c (same +0xd9 field, same shape) could only guess at —
this is a `bool` setter on a shared base/mixin class (`xy_system::bg::System`),
not an arbitrary byte field. Module's `.text` is file offset 0x180, size
0xd70 (per CRO segment table); only 2 named exports in this module
(`SetIsCallDraw`, `nnroControlObject_`). Several more tail-cluster
candidates spotted nearby for a future agent: `0xcf8` (lone `bx lr`,
weak-evidence no-op stub), `0xd70` (`mov r0,#1; bx lr` trivial stub),
`0xcfc`/`0xd34` (two near-identical dtor-idiom helpers — one with the
known-good member-load-after-vtable-load ordering at `0xcfc`, matching the
residue-class rule already documented in the toolchain doc; not attempted
this session), and `0xe40` (a selector-based switch with an
`ldrcc pc,[pc,r1,lsl#2]` jump table on 6 cases — same jump-table/
`--mask-ranges` caveat as `DllDialogCommon@0xdd8`, not attempted).
