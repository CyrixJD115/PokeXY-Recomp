Module: DllTownmap
Offset: 0x3240
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, return 0, ignores `this`.
Note: Mid-module stub (not near the import-veneer table). A one-word literal pool (0x323c) precedes it; an easy unclaimed follow-up sits right after at 0x3248 (`ldr r0,[r0,#4]; ldr r0,[r0]; bx lr`, size 0xc — double-indirection getter, `return *(*(this+4));`). Also unclaimed: bare `bx lr` weak-evidence stub at 0x1494 (literal pool precedes at 0x1490), and a double-field store helper around 0x3628 (`mov r2,r1; stm r0,{r1,r2}; bx lr`).
