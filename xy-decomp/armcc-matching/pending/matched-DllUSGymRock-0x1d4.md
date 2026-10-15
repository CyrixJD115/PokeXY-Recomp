Module: DllUSGymRock
Offset: 0x1d4
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: An eighteenth module. Same tail-of-import-veneer-table stub cluster
shape as DllUSDynamo/DllUSBadgeCheckGate/DllUS3DDun: a lone `bx lr` at
0x1d0, three identical `return 0` stubs at 0x1d4/0x1dc/0x1e4 (only 0x1d4
claimed), two more lone `bx lr` at 0x1ec/0x1f0, then a real
constructor-shaped function with VFP/float math at 0x1f4 (unattempted).
.text segment for this module is at file offset 0x180, size 0x4d8 (CRO
segment table at 0x2010, 6 entries).
