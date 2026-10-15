Module: DllPssMessageWindow
Offset: 0x3c0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`.
Note: A 23rd module. Sits at the tail of the import-veneer table (0x180-0x3bf):
two identical return-1 stubs at 0x3c0/0x3c8, then three bare `bx lr` stubs
(0x3d0/0x3d4/0x3d8, untried weak-evidence class), then the module's first
real function (a constructor with VFP math) at 0x3dc. Same recurring
trivial-stub-cluster shape as every other Dll module mined so far.
