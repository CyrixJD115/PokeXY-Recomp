Module: DllTradelist
Offset: 0xc04
Size: 0x38
Match: 14/14
One-line: Destructor: vptr install, free member @+0x14 if non-null, tail-calls a second import instead of returning `this`.
Note: Same idiom already proven in DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764 — good vtable-before-member load ordering, same 0x38-not-0x34 size gotcha (own literal pool at 0xc38). The neighboring sibling dtor at 0xc3c has the known-bad member-load-first ordering and was skipped on sight.
