Module: DllUSGymElec
Offset: 0x268
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: A thirty-sixth(+) module win, same recurring tail-of-import-veneer-table
stub cluster shape as DllUSGymRock/DllUSGymGrass/DllUSGymFairy/DllUSGymInsect
etc. 28 import veneers (0x180-0x25f), two lone `bx lr` weak-evidence stubs at
0x260/0x264, three identical `mov r0,#0; bx lr` stubs at 0x268/0x270/0x278
(only 0x268 claimed), two more lone `bx lr` at 0x280/0x284, then the module's
first real function (field getter with an early-out comparison) at 0x288.
Tried DllPokeViewer first (suggested starting module): its entire real .text
is exactly one function at 0x180 (the `nnroControlObject_` module-control
entry, a 5-case `cmp r1,#5; ldrcc pc,[pc,r1,lsl#2]` jump-table dispatcher with
a `push {r4,lr}`/`pop {r4,pc}` frame and a default-case tail that loads three
pool constants in a row interleaved with `e320f000`/`mov r0,r0` NOP-hint
words) -- this is the exact unresolved residue class already logged in
attempted.md (DllDialogCommon@0xdd8, DllFieldDebug's note on its module's
nnroControlObject_, DllCommWait@0xce0): known-bad push/pop-frame-with-no-calls
plus the relaxed-call NOP pattern in the default branch, never matched by any
prior agent. Abandoned it without attempting (no other candidate exists in
that tiny module) and picked DllUSGymElec instead per the "pick any other
unmined module" fallback instruction.
