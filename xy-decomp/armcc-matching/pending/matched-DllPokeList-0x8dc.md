Module: DllPokeList
Offset: 0x8dc
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A new (sixty-sixth) module mined. Sits in a run of near-identical
leaf stubs right after the import-veneer table (0x180-0x8db): `return 1`
stubs at 0x8dc/0x8e4/0x8ec/0x8f8 (only 0x8dc claimed), bare `bx lr`
weak-evidence stubs at 0x8f4/0x900/0x904/0x908/0x90c, then the module's
first real function (a `push {r4,r5,r6,r7,r8,lr}` constructor-shaped
dispatcher) at 0x910. Same recurring tail-of-import-veneer-table
leaf-stub cluster shape as every other Dll module mined so far.
