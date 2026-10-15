Module: DllTrainerCase
Offset: 0x37e4
Size: 0x38
Match: 14/14
One-line: Destructor — vptr install, free member @+0x14 if non-null, tail-call a second import instead of returning `this`.
Note: Same idiom already proven in DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764 — good vtable-literal-before-member-load ordering, own literal pool at 0x3818 pushes real size to 0x38 (not naive 0x34). A sibling destructor right after at 0x381c has the known-bad member-load-first scheduling order and was skipped on sight. New module for the swarm (DllTrainerCase was previously unmined). Module also has an easy trivial-stub cluster at 0x37a4-0x37e0 (return-1 and bare bx-lr stubs) and a return-0 stub at 0x3850 worth claiming next.
