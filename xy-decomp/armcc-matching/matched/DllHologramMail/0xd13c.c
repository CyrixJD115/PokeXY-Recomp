/* DllHologramMail function @ CRO file offset 0xd13c, size 0x24 — MATCHED (masked, 9/9).
 * Style: natural
 * Byte-identical shape to `0xd424` in this same module (file offset-only
 * difference in the masked `bl` target, which resolves to 0xd78 here vs
 * 0xd10 there): loads `this+36` (a pointer), dereferences it at +4 to get
 * an owned object, calls an import veneer with the function's 3rd argument
 * and that object, then stores the literal 11 into `this+40`. Reused the
 * `0xd424.c` candidate unmodified.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHologramMail 0xd13c 0x24 xy-decomp/armcc-matching/matched/DllHologramMail/0xd13c.c
 */

typedef struct {
    unsigned char pad0[36];
    void *f36;
    unsigned char f40;
} T;

extern void *import_d78(void *v, void *obj);

void fn_d13c(T *t, long arg1, void *arg2)
{
    void *obj = *(void **)((char *)t->f36 + 4);
    import_d78(arg2, obj);
    t->f40 = 11;
}
