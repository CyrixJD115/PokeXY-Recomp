Module: DllFriendEntry
Offset: 0x4c4
Size: 0x38
Match: 14/14
One-line: Destructor — vptr install, free member @+0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: 32nd module. Same idiom and same size gotcha (0x38 not 0x34, own literal pool at 0x4f8) already documented for DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c. Same "good" load ordering rule: vtable literal loaded into r1 before the member load into r0, store deferred until after both. The neighboring function at 0x4fc has the identical shape but member-load-first (known-bad scheduling order) — skipped on sight, not attempted.
