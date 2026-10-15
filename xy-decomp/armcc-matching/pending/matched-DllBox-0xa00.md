Module: DllBox
Offset: 0xa00
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence, sits between the `return 1` stub cluster and 0xa04.
Note: Weak-evidence match (any void no-side-effect function compiles to this one word); one of the run 0xa00/0xa08/0xa0c/0xa10 — only 0xa00 claimed.
