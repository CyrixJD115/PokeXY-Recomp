Module: DllGameClearSave
Offset: 0xd6c
Size: 0x38
Match: 14/14
One-line: Destructor — installs vtable, frees owned member @+0x14 if non-null, tail-calls a second import instead of returning `this` via pop{r4,pc}.
Note: Eighth module mined. Same dtor idiom and same member offset (+0x14) as DllFatalErrorProc@0x284/DllDialogCommon@0xcdc — vtable-literal load scheduled BEFORE the member load (the matchable ordering per the vptr-scheduling residue notes), confirming the idiom recurs in yet another unrelated module. Size trap: the function's own one-word literal pool (the vtable address) sits right at the 13/14-word boundary immediately before the next function at 0xda4; the real size is 0x38 (14 words, pool included), not 0x34 — matching only succeeded once the size was corrected to include it. Immediately followed in the disassembly by a near-identical sibling dtor at 0xda4 (member-load first, i.e. the known-bad scheduling order) and three bare `bx lr` stubs at 0xd60/0xd64/0xd68 just before this one — both left unattempted/untried for a future agent.
