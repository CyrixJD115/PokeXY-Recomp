/* DllSequence function @ CRO file offset 0x10a80, size 0x10 — MATCHED (masked, 4/4).
 * Style: trick(volatile-load) -- `volatile` on a field only to stop armcc narrowing/dropping a load
 * Third of the three untried "return 1"-shaped leads flagged in this
 * module's README entry for 0x1135c — the original lead quoted the
 * offset of just the trailing `mov r0,#1; bx lr` pair (0x10a88), but
 * the real function starts two words earlier at 0x10a80 (right after
 * the previous function's `pop {r4,r5,r6,pc}` at 0x10a7c) and is 0x10
 * bytes, not 0x8.
 *
 * Disassembly:
 *   ldr r0, [r0, #12]       ; this->inner  (a pointer field at +0xc)
 *   ldr r0, [r0, #204]      ; inner->field0xcc  (result discarded)
 *   mov r0, #1
 *   bx  lr
 *
 * The two leading loads chase a pointer two levels deep and the result
 * is never used anywhere (overwritten by the `mov r0,#1` right after) —
 * a genuine "touch a field then unconditionally return true" idiom
 * distinct from every residue class logged so far. A plain non-volatile
 * int field gets the loads optimized away entirely by armcc 5.04u5
 * -O2 -Otime (0/4, candidate collapses to the bare 2-word stub); marking
 * the leaf field `volatile` is what forces armcc to retain both chained
 * loads and reproduces the target exactly. The source likely really does
 * read a volatile/hardware-adjacent flag here (purpose unclear — maybe a
 * "touch to trigger a side effect" accessor) and discards it, always
 * reporting success.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0x10a80 0x10 xy-decomp/armcc-matching/matched/DllSequence/0x10a80.c
 */

struct Inner
{
    unsigned char pad[0xcc];
    volatile int field0xcc;
};

struct Outer
{
    unsigned char pad[0xc];
    struct Inner *inner;
};

int touch_and_true(struct Outer *this)
{
    int unused = this->inner->field0xcc;
    (void)unused;
    return 1;
}
