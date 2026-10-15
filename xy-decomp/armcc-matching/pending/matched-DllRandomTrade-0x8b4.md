Module: DllRandomTrade
Offset: 0x8b4
Size: 0x38
Match: 14/14
One-line: Destructor — installs a new vtable, frees an owned member at +0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: First match in this module (a seventy-seventh module). Same proven idiom as DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764 — good vtable-literal-before-member-load ordering, real size 0x38 (not naive 0x34) because the function's own literal pool word sits right before the next function at 0x8ec. Unclaimed nearby leads: six trivial leaf stubs immediately precede it (0x890/0x898 = `mov r0,#1; bx lr`, 0x8a0-0x8b0 = four bare `bx lr` weak-evidence stubs), and a near-identical sibling dtor-shaped function follows at 0x8ec (same shape, member-load-vs-vtable-load order not yet checked — could be the known-bad ordering, worth checking on sight before attempting).
