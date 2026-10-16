/* DllTownmap function @ CRO file offset 0x46c, size 0x1c.
 *
 * FIRST MATCHED FUNCTION in DllTownmap, a new module. Compiled with
 * armcc 5.04u5, --cpu=MPCore --arm --apcs=/interwork -O2 -Otime: every
 * instruction matches the original byte-for-byte modulo the single b/bl
 * branch immediate (a link-layout fact).
 * Style: natural
 *
 * Verify with:
 *   python3 tools/fingerprint_fn.py DllTownmap 0x46c 0x1c \
 *       matched/DllTownmap/0x46c.c
 * Match: 7/7 words (*** MASKED MATCH ***).
 *
 * Shape: the same conditional-accessor idiom already proven in
 * DllLangSelect@0x5b0 / DllDendouDemo@0x798 / DllBoxSearch@0x5b8 /
 * DllDendouPC@0x650 / DllEggDemo@0x568 / DllPCMenu@0x6b8: calls an
 * import veneer (CRO offset 0x180, real symbol unresolved) on `this`;
 * if it returns nonzero, returns the object's member at +0x10,
 * otherwise returns the literal constant 3 (not the veneer's own
 * result -- armcc re-materializes the constant for the false branch
 * rather than reusing a register). Struct is a placeholder: only the
 * +0x10 field offset is proven.
 */

extern int Fn180(void *a0);

struct S {
    unsigned char pad[16];
    int field10; /* +0x10 */
};

int func_46c(struct S *a0)
{
    if (Fn180(a0) != 0) {
        return a0->field10;
    }
    return 3;
}
