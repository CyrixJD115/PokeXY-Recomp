Module: DllPuzzle
Offset: 0x1e90
Size: 0x14
Match: 5/5
One-line: post-indexed init: *o=v; callee(o+0x18); return callee_result-0x18
Note: retail has no r4 save: callee returns its arg, so write `return (char*)fn(p)-0x18` with fn returning void* (saving o in a local gives mov r4,r0 mismatch). Real size 0x14 not 0x18 (trailing word is padding)
