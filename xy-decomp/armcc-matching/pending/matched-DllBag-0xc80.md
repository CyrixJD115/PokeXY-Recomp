Module: DllBag
Offset: 0xc80
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: 42nd+1 module (DllBag, not yet in README's running count). Sits right
at the tail of the import-veneer table (0x180-0xc7f): a run of six
near-identical 1-2 word leaf stubs follows (0xc80/0xc88/0xc90 = `return 1`,
0xc98/0xca0/0xca4/0xca8 = bare `bx lr` weak-evidence stubs, all unclaimed),
then the module's first real function (a 9-register-push constructor-shaped
dispatcher) starts at 0xcac. Same recurring tail-of-import-veneer-table
leaf-stub cluster shape documented for every other Dll module mined so far.
