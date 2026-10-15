Module: DllTheEnd
Offset: 0x750
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1` (ignores `this`).
Note: A thirteenth module. Isolated between two unrelated helper functions
in the tail-of-vtable stub cluster already documented for every other
mined module (literal pool word at 0x74c belongs to the previous
function's epilogue, next function starts immediately at 0x758). Same
shape as DllNumberInput@0x1888 and the other `return 1` stubs.
