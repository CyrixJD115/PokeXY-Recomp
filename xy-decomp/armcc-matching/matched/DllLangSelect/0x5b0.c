/* DllLangSelect function @ CRO file offset 0x5b0, size 0x1c.
 * Style: natural
 *
 * FIRST MATCHED FUNCTION in DllLangSelect (2026-10-15), proving the
 * technique generalizes to a fifth module. Compiled with armcc 5.04u5,
 * --cpu=MPCore --arm --apcs=/interwork -O2 -Otime: every instruction
 * matches the original byte-for-byte modulo the single b/bl branch
 * immediate (a link-layout fact).
 *
 * Verify with:
 *   python3 tools/fingerprint_fn.py DllLangSelect 0x5b0 0x1c \
 *       matched/DllLangSelect/0x5b0.c
 * Match: 7/7 words (*** MASKED MATCH ***).
 *
 * Shape: a conditional accessor. Calls an import veneer (CRO offset
 * 0x1b0, real symbol unresolved) on `this`; if it returns nonzero,
 * returns the object's member at +0x14, otherwise returns the literal
 * constant 2 (not the veneer's own result — armcc re-materializes the
 * constant for the false branch rather than reusing a register).
 * Struct is a placeholder: only the +0x14 field offset is proven.
 */

extern int Fn1b0(void *a0);

struct S {
    unsigned char pad[20];
    int field20; /* +0x14 */
};

int func_5b0(struct S *a0)
{
    if (Fn1b0(a0) != 0) {
        return a0->field20;
    }
    return 2;
}
