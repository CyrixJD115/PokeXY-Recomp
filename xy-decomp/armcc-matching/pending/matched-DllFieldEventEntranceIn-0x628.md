Module: DllFieldEventEntranceIn
Offset: 0x628
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A new (fifty-first) module. Sits mid-module (not right after the
import-veneer table, which runs 0x180-0x2df / 44 entries): a literal-pool
word precedes it at 0x624, two bare `bx lr` weak-evidence stubs follow at
0x630/0x634, and a boolean getter (`push {r4,lr}; ldr r0,[r0,#24]; cmp;
beq/movne; pop {r4,pc}`) starts right after at 0x638. Same recurring
trivial-stub-cluster shape documented for every other mined Dll module.
