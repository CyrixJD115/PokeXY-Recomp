Module: DllVSDemo
Offset: 0x408
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence, see note.
Note: One of three bare `bx lr` single-word stubs (0x408/0x40c/0x410) right
after the five `return 1` stubs at 0x3e0-0x400. Weak-evidence class per
the toolchain doc: any no-op void function compiles to this same word.
