Module: DllLiveCup
Offset: 0x764
Size: 0x38
Match: 14/14
One-line: Destructor — installs a new vtable, frees an owned member at +0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: A new (sixty-third+) module confirming the recurring dtor idiom already proven in DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0: same "good" load ordering (vtable literal into r1 before the member load into r0, store deferred past both loads), same size gotcha (own literal-pool word at 0x798 pushes real size to 0x38, not the naive 0x34). The sibling function immediately following at 0x79c has the same shape but the opposite (member-load-first) known-bad ordering and was skipped on sight. Two trivial `return 1` stubs at 0x738/0x740/0x748 and five bare `bx lr` weak-evidence stubs at 0x750-0x760 precede it in the same stub cluster, right after the import-veneer table (0x180-0x737); none of those claimed yet.
