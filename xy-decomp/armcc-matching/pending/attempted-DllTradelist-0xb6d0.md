Module: DllTradelist
Offset: 0xb6d0 (also tried the neighboring 0xb6e4, same residue)
Size: 0x14 each
What it is: two adjacent struct-copy helper functions, no branches/calls.
  0xb6d0: `ldr r2,[r1]; str r2,[r0]; ldrh r3,[r1,#4]; strh r3,[r0,#4]; bx lr`
    (copies a 4-byte field then a 2-byte field).
  0xb6e4: `ldr r2,[r1]; ldr r3,[r1,#4]; str r2,[r0]; str r3,[r0,#4]; bx lr`
    (loads both fields, then stores both — two 4-byte fields).
What was tried: field-by-field assignment (`dst->a = src->a; dst->b =
src->b;`), reversed field order, `void*` params with local casts,
unsigned vs signed short, whole-struct assignment (`*dst = *src;` —
over-merges into `ldm`/`stm`), and a volatile-source variant that got
the load-both-then-store-both shape right for 0xb6e4 but still picked
the wrong registers/order.
Stuck on: armcc 5.04u5 always reuses r1 (the `src` pointer register,
dead after its last load) for the second temp instead of allocating r3,
and/or orders the second store before the first. Every C idiom tried
either reproduces the retail register choice or the retail store order,
never both at once. This is the same general residue class already
documented for `DllConfig@0x31b8` (struct-copy store-merge/
register-allocation quirks) in `attempted.md` — a case of "two adjacent
struct-copy helpers, same symptom" rather than a new independent lead.
Best ratio reached: 3/5 (0xb6d0), 2/5 (0xb6e4).
Lead for next time: try passing the struct by value (not pointer) to
see if parameter-register pressure changes allocation, or try an
explicit `register` hint / splitting into a helper that copies through
an intermediate local array.
