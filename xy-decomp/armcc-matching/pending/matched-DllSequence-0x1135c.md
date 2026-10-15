Module: DllSequence
Offset: 0x1135c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in DllSequence (seventy-first module). Sits inside a run of
near-identical 1-2 word leaf stubs (0x1134c-0x1136f): bare `bx lr` stubs at
0x1134c/0x11350/0x11354/0x11358, this `return 1` stub, more bare `bx lr`
stubs at 0x11360/0x11364/0x11368/0x1136c. Right after the previous
function's one-word literal pool at 0x11344-0x11348. DllSequence has an
unusually large import-veneer table (.text runs 0x180-0x1244c, veneer
table alone roughly 0x180-0xec0); real functions only start well past
that. Two more unclaimed `return 1` leads of the exact same shape were
spotted in the disassembly at 0xee90 and 0xef98 (same cluster idiom,
near the veneer table's own tail) and at 0x10a88 (mid-module) — all four
candidates found via a script scanning for `mov r0,#1 / bx lr` pairs in
the objdump output; only 0x1135c was attempted/verified this run.
