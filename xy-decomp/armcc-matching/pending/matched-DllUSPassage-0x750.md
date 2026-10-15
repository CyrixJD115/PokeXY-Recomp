Module: DllUSPassage
Offset: 0x750
Size: 0x8
Match: 2/2
One-line: Trivial field getter, returns the int field at `this+0xb4`.
Note: Prologue-less leaf, no branches/calls. Sits right after a larger
destructor/cleanup-loop function (0x64c-0x74c) and right before another
large `push {r4,...,lr}` function at 0x758.
