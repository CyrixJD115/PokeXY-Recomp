Module: DllTradeDemo
Offset: 0x644
Size: 0x24
Match: 9/9
One-line: Prologue-less leaf predicate — double-indirects through a +24 pointer field to a Leaf object's +56 byte flag; if the flag is 1, stores 5 into +16 and returns 2, else returns 4 (no store).
Note: Immediately follows DllTradeDemo@0x550's own literal pool (one word at 0x640); the next function at 0x668 (push {r4,lr}, calls an import twice gated on null-checks) looks like another tractable candidate for a future agent on this module.
