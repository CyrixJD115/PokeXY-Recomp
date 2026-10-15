Module: DllBattleHouseConnect
Offset: 0x2fc
Size: 0x38
Match: 14/14
One-line: Destructor — installs vtable pointer, frees a member at +0x14 if non-null, tail-calls a second import instead of returning `this` directly.
Note: Eighth module mined. Same idiom already proven at DllFatalErrorProc@0x284 (pool-load-into-r1 before member-load-into-r0, store deferred, then `pop {r4,lr}; b <import>` tail call). Size gotcha repeats too: the naive 0x34 guess (matching the non-tail-call sibling shape) misses the function's own literal-pool word at file offset 0x330 — real size is 0x38, one word more, right up against the next function's `push {r4,lr}` at 0x334.
