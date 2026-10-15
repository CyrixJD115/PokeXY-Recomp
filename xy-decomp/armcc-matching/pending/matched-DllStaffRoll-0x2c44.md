Module: DllStaffRoll
Offset: 0x2c44
Size: 0x38
Match: 14/14
One-line: Destructor — vtable install, free member @+0x14 if non-null, tail-call a second import instead of returning `this` directly.
Note: Same idiom already proven in DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764 — adds DllStaffRoll to that idiom's module list. Same size gotcha (own literal pool word at 0x2c78 pushes real size to 0x38, not 0x34). The immediate sibling function at 0x2c7c has the same shape but the known-bad member-load-first scheduling order — skipped on sight. Also two unclaimed trivial stubs right after it: 0x2cb0 (SetIsCallDraw-shaped byte setter — see separate pending note) and 0x2cb8 (`return 0`).
