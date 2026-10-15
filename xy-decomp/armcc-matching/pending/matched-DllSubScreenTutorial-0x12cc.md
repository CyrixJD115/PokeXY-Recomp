Module: DllSubScreenTutorial
Offset: 0x12cc
Size: 0x38
Match: 14/14
One-line: Destructor -- vptr install, free member @+0x14 if non-null, tail-call a second import.
Note: Same idiom already proven in DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764. Same size gotcha (own literal pool word pushes real size to 0x38, not 0x34). An unclaimed near-identical sibling (pop{r4,pc} return instead of tail-call) sits right next door at 0x1304.
