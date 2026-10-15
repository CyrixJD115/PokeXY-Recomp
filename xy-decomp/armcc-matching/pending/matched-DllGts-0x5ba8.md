Module: DllGts
Offset: 0x5ba8
Size: 0x1c
Match: 7/7
One-line: double-pointer-deref call+return helper — `fn4f4(**field10, field30)`, returns 0x1b.
Note: load-latency-hiding scheduling trap. armcc's retail build does a pre-indexed load (`ldr r2,[r0,#16]!`) to fetch field10's value while advancing the base, a plain offset load off the advanced base for field30, THEN the second dereference last — writing it as one expression or a manually-advanced cursor pointer produces plain non-writeback loads instead (3/7 off). Fix: split into separate local-variable statements in this exact order (field10-value first, field30 second, final deref inlined in the call).
