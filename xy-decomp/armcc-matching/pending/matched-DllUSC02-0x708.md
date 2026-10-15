Module: DllUSC02
Offset: 0x708
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: A 28th module. Sits mid-module (not at the import-veneer-table tail
or module head like most prior stub-cluster finds): right after a
constructor-shaped function ending at 0x6fc (`pop {r4, pc}`) plus its own
2-word literal pool at 0x700/0x704, and right before a bare `bx lr`
no-op stub at 0x710 and a one-line accessor/tail-call at 0x714
(`ldr r0,[r0,#32]; b <veneer>`). Confirms the recurring trivial-stub
cluster shape isn't confined to the head/tail of a module's vtable
region. (Also investigated DllUSC02@0x348 — a getter-with-conditional-
call shape — but its disassembly ends in `pop {r4, lr}` with no
following branch/`bx lr`/`pop {..,pc}` before the next function's
prologue at 0x368, which doesn't look like a well-formed function
boundary; left unattempted rather than guess wrong, flagging for anyone
who wants to dig into it with more time.)
