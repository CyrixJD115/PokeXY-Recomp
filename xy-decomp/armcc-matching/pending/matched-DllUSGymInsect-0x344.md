Module: DllUSGymInsect
Offset: 0x344
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0` (mov r0,#0; bx lr).
Note: Thirty-second new module. Same recurring tail-of-import-veneer-table
leaf-stub cluster seen everywhere: a call-through wrapper at 0x318 (push
{lr}; sub sp,#12; bl veneer; pop {pc}), a lone `bx lr` at 0x340, this
`return 0` stub, two more identical `return 0` stubs at 0x34c/0x354
(unclaimed), two more lone `bx lr` at 0x35c/0x360, then a real
VFP-heavy dispatcher function at 0x364 (unattempted, uses float compares
and an indexed lookup table -- not a trivial win).
