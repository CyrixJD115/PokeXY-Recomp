/* DllPuzzle function @ CRO file offset 0xae8, size 0x1c.
 * Style: natural
 *
 * FIRST MATCHED FUNCTION in DllPuzzle, a new module. Compiled with
 * armcc 5.04u5, --cpu=MPCore --arm --apcs=/interwork -O2 -Otime:
 * matches the original byte-for-byte (no branch immediates to mask --
 * this specimen has no bl/b at all, just a trailing `bx lr`).
 *
 * Verify with:
 *   python3 tools/fingerprint_fn.py DllPuzzle 0xae8 0x1c \
 *       matched/DllPuzzle/0xae8.c
 * Match: 7/7 words (*** MASKED MATCH ***).
 *
 * Shape: a conditional field setter, prologue-less leaf, no calls.
 * Chases two pointer members off `this` (`this->obj1->obj2`), reads a
 * byte field there, and if it equals 1, writes the literal 2 into a
 * byte field on `this` itself. Void return. Struct layout is a
 * placeholder padded to the proven offsets only (+4 obj1 ptr, +8
 * output byte on `this`; +0x2c obj2 ptr on obj1, +0x60 input byte on
 * obj2).
 */

struct Obj2 {
    unsigned char pad[0x60];
    unsigned char val60; /* +0x60 */
};

struct Obj1 {
    unsigned char pad[0x2c];
    struct Obj2 *obj2; /* +0x2c */
};

struct This {
    unsigned char pad0[4];
    struct Obj1 *obj1; /* +4 */
    unsigned char out8; /* +8 */
};

void func_ae8(struct This *this)
{
    if (this->obj1->obj2->val60 == 1) {
        this->out8 = 2;
    }
}
