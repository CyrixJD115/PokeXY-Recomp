Module: DllGameSync
Offset: 0x5e0
Size: 0x38
Match: 14/14
One-line: Destructor — installs a new vtable, frees an owned member at +0x14 if non-null, then tail-calls a second import instead of returning `this` directly.
Note: First matched function in a new (fifty-fifth) module. Identical idiom to DllFatalErrorProc@0x284 / DllCommendation@0xc94 / DllBattleHouseConnect@0x2fc / DllGameClearSave@0xd6c / DllFriendEntry@0x4c4 / DllEshop@0x554 (same "good" vtable-literal-before-member-load scheduling, same size gotcha — own literal pool word at 0x614 pushes real size to 0x38, not 0x34). The module's import-veneer table runs 0x180-0x5c4 (139 entries); a trivial `mov r0,#1; bx lr` stub sits at 0x5c8 (unclaimed, easy next pick for another agent) followed by five bare `bx lr` weak-evidence stubs at 0x5cc-0x5dc. The sibling function immediately after this one at 0x618 has the same dtor shape but the known-bad member-load-before-vtable-load scheduling order and was skipped on sight. Real code continues after the literal pool at 0x614, with a dispatcher-shaped function at 0x618 and more real functions through at least 0x700+ (unexplored, plenty left for future agents).
