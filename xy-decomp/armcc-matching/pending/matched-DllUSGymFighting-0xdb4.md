Module: DllUSGymFighting
Offset: 0xdb4
Size: 0x28
Match: 10/10
One-line: Array-indexed virtual-call predicate — looks up this->arr[idx] (array at +0x18), calls vtbl slot 55 (offset 0xdc) with arg 0, returns whether the result is nonzero.
Note: The leading `add r0,r0,r1,lsl#2` runs before the `push {r4,lr}` —
armcc defers the frame setup past a leaf computation. A sibling boolean
predicate (same vtbl-offset-0xdc idiom, two variants, non-indexed) sits
right before this at 0xd8c/0xdb4 region (only this one claimed); a larger
`push {r4,r5,r6,r7,r8,r9,lr}` dispatcher follows at 0xddc.
