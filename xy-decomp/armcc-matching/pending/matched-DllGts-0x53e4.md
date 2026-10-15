Module: DllGts
Offset: 0x53e4
Size: 0x1c (the hint's 0x24 included 8 bytes of unrelated dead/padding before the next function — corrected)
Match: 7/7
One-line: third sibling of 0x5ba8/0x5bc4 — same double-deref call+return body, returns 0x13.
Note: the candidate scanner's claimed size 0x24 is wrong for fingerprinting purposes; the real function (matching Ghidra's own boundary) ends 0x1c bytes in, at the `pop {r4,pc}`. The trailing `sub r0,r0,#8` + nop belong to padding before the next function's prologue at 0x5408, not this function. See 0x5ba8's pending note for the load-latency-hiding scheduling trap shared by all three siblings.
