Module: DllPssIconSelect
Offset: 0x540
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Mined right after the module's import-veneer table (0x180-0x53f,
120 entries). A twin `return 1` stub sits at 0x548, and three bare
`bx lr` single-word stubs follow at 0x550/0x554/0x558 (weak-evidence
class, unclaimed) before the module's first real function at 0x55c
(a dispatcher with a masked `bl` call). This is a new (71st) module
for the swarm's match table.
