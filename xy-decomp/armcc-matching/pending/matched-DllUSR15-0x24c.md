Module: DllUSR15
Offset: 0x24c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: A thirty-second module mined. Same recurring tail-of-import-veneer-table
stub cluster shape: import veneers run 0x180-0x247, a lone `bx lr` at 0x248,
three identical `mov r0,#0; bx lr` stubs at 0x24c/0x254/0x25c (only 0x24c
claimed), three more lone `bx lr` at 0x260/0x264/0x268, then a real
VFP-using function at 0x26c (not attempted).
