DllStatus 0x459c (size 0xb8, 46 words): best 26/46, differs only in the byte-toggle region.
Target: `ldrb [r4,#245]; cmp #0; beq L1; i2; i4; ldrb; cmp #1; moveq r0,#0; beq L2; L1: mov r0,#1; L2: strb r0,[r4,#245]`,
and later `mov r0,#1; mov r3,r0; str r0,[sp]` re-materializes constants (no hoisting, prologue push {r3,r4,r5,lr}).
Tried: if/else, ternary toggle, local n var, goto join -- armcc always hoists #1/#0 into r5/r6 (push grows). Need a shape that keeps constants unhoisted.
Everything else (own call, vtable slot 2 call via this+0x58, ldrd arg loads, i6/i7 calls) matched.
