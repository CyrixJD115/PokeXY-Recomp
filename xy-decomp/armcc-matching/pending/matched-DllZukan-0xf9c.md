Module: DllZukan
Offset: 0xf9c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A new (67th-ish) module. Sits in the recurring tail-of-import-veneer-table
leaf-stub cluster (0xf98-0xfd7): veneer table runs 0x180-0xf97, a bare `bx lr`
precedes it at 0xf98, two more `return 1` twins follow at 0xfa4/0xfac
(unclaimed), two bare `bx lr` at 0xfb4/0xfb8, a fourth `return 1` stub at
0xfbc (unclaimed), five more bare `bx lr` weak-evidence stubs through
0xfd7, then the module's first real function (dtor/init helper) at 0xfd8.
