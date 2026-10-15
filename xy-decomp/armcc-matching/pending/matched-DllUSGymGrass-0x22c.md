Module: DllUSGymGrass
Offset: 0x22c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: An eighteenth module. Sits in the standard tail-of-import-veneer stub
cluster: a lone `bx lr` at 0x228, three `mov r0,#0; bx lr` stubs at
0x22c/0x234/0x23c (only 0x22c claimed), then three more lone `bx lr`
stubs at 0x244/0x248/0x24c, before the module's first real function
(a constructor-shaped push {r4-r8,lr}) begins at 0x250.
