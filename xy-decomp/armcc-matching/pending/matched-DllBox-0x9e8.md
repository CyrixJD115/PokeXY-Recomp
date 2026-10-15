Module: DllBox
Offset: 0x9e8
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in a 38th module. Sits right after the import-veneer
table (0x180-0x9e7): three identical `return 1` stubs at
0x9e8/0x9f0/0x9f8 (only 0x9e8 claimed), a `return 1` stub at 0xa04, and
bare `bx lr` weak-evidence stubs at 0xa00/0xa08/0xa0c/0xa10, before the
module's first real function (a 10-register-push dispatcher) at 0xa14.
Same recurring tail-of-import-veneer-table leaf-stub cluster shape as
every other Dll module mined so far.
