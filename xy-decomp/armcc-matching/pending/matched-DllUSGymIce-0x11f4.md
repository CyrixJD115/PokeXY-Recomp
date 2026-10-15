Module: DllUSGymIce
Offset: 0x11f4
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence.
Note: Preceded by a 4-word literal pool (0x11e4-0x11f0) belonging to the
previous function (ends 0x11e0, `pop {r4,r5,pc}`); the next function
starts right after at 0x11f8 with `add r1,r1,#4` before its own
`push {r4,lr}`.
