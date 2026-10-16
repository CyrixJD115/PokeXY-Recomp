/* DllHologramMail function @ CRO file offset 0xd424, size 0x24 — MATCHED (masked, 9/9).
 * Style: natural
 * Loads `this+36` (a pointer), dereferences it at +4 to get an owned
 * object, calls an import veneer (file offset 0xd10, target masked) with
 * the function's own 3rd argument (the second real parameter after
 * `this`; the 2nd argument, r1, is unused entirely) and that object, then
 * stores the literal 11 into `this+40` (byte field). Void return.
 *
 * An identical-shape sibling exists at file offset 0xd13c (same module,
 * different masked `bl` target) — reused this exact candidate unmodified
 * for that second match too.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHologramMail 0xd424 0x24 xy-decomp/armcc-matching/matched/DllHologramMail/0xd424.c
 */

typedef struct {
    unsigned char pad0[36];
    void *f36;
    unsigned char f40;
} T;

extern void *import_d10(void *v, void *obj);

void fn_d424(T *t, long arg1, void *arg2)
{
    void *obj = *(void **)((char *)t->f36 + 4);
    import_d10(arg2, obj);
    t->f40 = 11;
}
