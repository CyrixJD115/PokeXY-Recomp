Module: DllPssCharaMenu
Offset: 0x5e0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Seventieth module mined. Sits right after the import-veneer table
(0x180-0x5df): an identical twin `return 1` stub at 0x5e8 (unclaimed),
two bare `bx lr` weak-evidence stubs at 0x5f0/0x5f4, then the module's
first real function (a `push {r4,r5,r6,r7,r8,lr}` dispatcher) at 0x5f8.
Same recurring tail-of-import-veneer-table leaf-stub cluster seen in
every other Dll module mined so far.
