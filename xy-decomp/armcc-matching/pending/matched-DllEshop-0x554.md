Module: DllEshop
Offset: 0x554
Size: 0x38
Match: 14/14
One-line: Destructor — installs a new vtable, frees owned member at +0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: A new (fifty-first-ish) module, mined from the stub cluster right after the import-veneer table (0x180-0x52f). Identical idiom to DllFatalErrorProc@0x284 / DllCommendation@0xc94 / DllBattleHouseConnect@0x2fc / DllGameClearSave@0xd6c / DllFriendEntry@0x4c4: same "good" load ordering (vtable literal into r1 before the member load into r0, store deferred past both), same size gotcha (own literal pool word at 0x588 pushes real size to 0x38, not the naive 0x34). Two trivial `mov r0,#1; bx lr` stubs at 0x530/0x538 and six bare `bx lr` weak-evidence stubs at 0x53c-0x550 precede it (unclaimed). A sibling dtor-shaped function immediately follows at 0x58c (same shape, not yet checked for load-order — worth a quick look for a next agent).
