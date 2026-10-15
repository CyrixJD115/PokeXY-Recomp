Module: DllFieldEventPokeRideAction
Offset: 0x520
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this` — a new (fifty-fifth) module.
Note: Sits mid-module rather than at the tail of the import-veneer table: a
field-copy-and-store helper (`add r0,r0,#0x24; stm r0,{r1,r2}; bx lr`, file
offset 0x514, 0xc bytes, unattempted — looks tractable as a next easy win)
ends right before it, and a real `push {r4,r5,r6,lr}`-prologue function
starts right after at 0x528. Same recurring trivial-stub shape as
`DllBox@0x9e8`/`DllFieldEventEntranceIn@0x628`/`DllFieldEventPlayer@0x624`.
