Module: DllUSPokecen
Offset: 0x24c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0` (ignores `this`).
Note: An eighteenth module. Import-veneer table runs 0x180-0x23f; three
bare `bx lr` stubs follow at 0x240/0x244/0x248, then three identical
`mov r0,#0; bx lr` stubs at 0x24c/0x254/0x25c (only 0x24c claimed here —
0x254/0x258 untried), another bare `bx lr` at 0x264, then the module's
first real function (a constructor with VFP/float setup) starts at 0x268.
Same recurring trivial-stub-cluster shape documented for every other
mined module.
