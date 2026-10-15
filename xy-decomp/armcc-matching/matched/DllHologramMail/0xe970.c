/* DllHologramMail function @ CRO file offset 0xe970, size 0x24 — MATCHED (masked, 9/9).
 * Style: trick(static-helper) -- one-use static helper only to pin an argument register
 * Calls an import veneer (file offset 0x190, target masked) with `this+8`
 * (a pointer field) and the literal 0, then sets two adjacent byte flags
 * (`this+17`, `this+16`, in that store order) to the literal 2. Void return.
 *
 * Non-obvious part: the retail binary holds the constant 2 in r1 (not r0,
 * which is dead after the call's return value goes unused) for both strb
 * instructions. A flat `t->f17 = 2; t->f16 = 2;` after the call makes armcc
 * 5.04u5 reuse the dead r0 instead, 6/9 masked match. Routing the two
 * stores through a separate (un-inlined-in-source, auto-inlined-by-the-
 * compiler) static helper `set_flags(T*, unsigned char)` makes the second
 * AAPCS parameter land in r1 at the call site and armcc keeps it there for
 * both stores once inlined — full 9/9 match. Same "wrap the stores in a
 * tiny static helper to pin the argument register" idiom documented
 * elsewhere in this project's residue notes (see `0x18dc`/field-copy
 * helpers in DllNumberInput).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHologramMail 0xe970 0x24 xy-decomp/armcc-matching/matched/DllHologramMail/0xe970.c
 */

typedef struct {
    unsigned char pad0[8];
    void *f8;
    unsigned char pad1[4];
    unsigned char f16;
    unsigned char f17;
} T;

extern void *import_190(void *p, long v);

static void set_flags(T *t, unsigned char v)
{
    t->f17 = v;
    t->f16 = v;
}

void fn_e970(T *t)
{
    import_190(t->f8, 0);
    set_flags(t, 2);
}
