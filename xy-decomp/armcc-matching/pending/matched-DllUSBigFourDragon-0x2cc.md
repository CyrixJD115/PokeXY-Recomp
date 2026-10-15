Module: DllUSBigFourDragon
Offset: 0x2cc
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: Twenty-eighth module mined. Same recurring tail-of-import-veneer-table
leaf-stub cluster shape as every other Dll module: a lone `bx lr` precedes
it at 0x2c8, two more identical `return 0` stubs follow at 0x2d4/0x2dc
(unclaimed), two more bare `bx lr` at 0x2e4/0x2e8, then a prologue-less
tail-virtual-call leaf at 0x2ec-0x300 (unclaimed), another bare `bx lr` at
0x304, then the module's first real function (push {r4,r5,r6,lr}) at 0x308.
