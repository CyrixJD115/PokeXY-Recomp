Module: DllVSDemo
Offset: 0x3e0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First real function right after the import-veneer table (0x180-0x3df),
a seventy-first independent module hitting this recurring stub-cluster
shape. Four more near-identical candidates sit right next to it:
twin `return 1` stubs at 0x3e8/0x3f0/0x3f8/0x400, and three bare `bx lr`
weak-evidence stubs at 0x408/0x40c/0x410 — all unclaimed.
