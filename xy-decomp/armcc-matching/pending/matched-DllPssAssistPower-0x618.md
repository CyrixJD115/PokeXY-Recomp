Module: DllPssAssistPower
Offset: 0x618
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Sits right after the import-veneer table (0x180-0x617) in a cluster
of near-identical leaf stubs: four `mov r0,#1; bx lr` stubs at
0x618/0x620/0x628/0x630/0x640 (only 0x618 claimed), interleaved with bare
`bx lr` weak-evidence stubs at 0x638/0x63c/0x648/0x64c, then the module's
first real function (a 7-case jump-table dispatcher via `ldrcc pc,[pc,r0,lsl#2]`,
same shape as other mined dispatcher candidates) at 0x650. This is a new
(seventieth) module for the swarm.
