Module: DllZukan
Offset: 0x1be64
Size: 0x8
Match: 2/2
One-line: Trivial field getter, returns the long at this+0x8.
Note: Third and last of the run of three consecutive 2-word
prologue-less getters (0x1be54 +0x28, 0x1be5c +0xc, 0x1be64 +0x8). The
very next instruction group at 0x1be6c is an array-indexed getter
(`add r0,r0,r1,lsl#2; ldr r0,[r0,#56]; bx lr`, size 0xc) — untried, looks
tractable as an easy follow-up (return ((T*)this)->arr[idx].fieldN
pattern), and a `mov r0,#1`-style stub cluster starts right after at
0x1be78 (untried). Also unclaimed in this module: two more of the three
near-identical `return 1` stubs in the 0xf98-0xfd7 cluster (0xfa4 and
0xfbc), plus several bare `bx lr` weak-evidence stubs in the same
cluster (0xf98/0xfb4/0xfb8/0xfc4/0xfc8/0xfcc/0xfd0/0xfd4).
