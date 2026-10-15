Module: DllUSMirror
Offset: 0x1cc
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0` (ignores `this`).
Note: Eighteenth module mined. `.text` @ file offset 0x180 (+0x308). Import
veneer table runs 0x180-0x1c7 (8 veneers). Stub cluster at 0x1c8-0x1fc:
a lone `bx lr` at 0x1c8, three identical `return 0` stubs at
0x1cc/0x1d4/0x1dc (only 0x1cc claimed here), three more lone `bx lr` at
0x1e4/0x1e8/0x1ec, then a setter-through-indirection
(`ldr r0,[r0,#12]; str r1,[r0]; bx lr`) at 0x1f0, another lone `bx lr` at
0x1fc, then a real dispatcher function starting at 0x200. Same recurring
trivial-stub shape as every other Dll module mined so far.
