Module: DllTrialDownload
Offset: 0x728
Size: 0x38
Match: 14/14
One-line: Destructor — installs a new vtable, frees owned member at +0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: Same recurring idiom as DllFatalErrorProc@0x284/DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764 — "good" load ordering (vtable literal into r1 before the member load), real size 0x38 (own literal pool word at 0x75c, right before the next function at 0x760). A near-identical sibling destructor sits right after at 0x760 but uses the known-bad member-load-first ordering (push {r4,lr}; mov r4,r0; ldr r0,[r0,#20]; ldr r1,[pc,#28]@0x790; cmp r0,#0; str r1,[r4]; ...) — skipped on sight, not attempted.
