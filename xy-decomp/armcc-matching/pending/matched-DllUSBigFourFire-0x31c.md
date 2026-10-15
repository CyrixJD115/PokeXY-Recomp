Module: DllUSBigFourFire
Offset: 0x31c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: A twenty-eighth module. Same recurring tail-of-import-veneer-table
leaf-stub cluster shape as every other Dll module mined so far: import
veneers run 0x180-0x317, a lone `bx lr` stub at 0x318, three identical
`mov r0,#0; bx lr` stubs at 0x31c/0x324/0x32c (only 0x31c claimed), a
lone `bx lr` at 0x334, then the module's first real function (a
notification dispatcher, same shape as DllNumberInput@0x1890 /
DllPssPlaySelect@0x2e4) starts at 0x338.
