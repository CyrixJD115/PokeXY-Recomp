Module: DllUSKinomi
Offset: 0xf44
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence (any side-effect-free void function compiles to the same word).
Note: Second of two lone `bx lr` stubs (0xf44/0xf48, only 0xf44 claimed) between the forwarder@0xf40 and the `return 0` stub run starting at 0xf4c.
