/*
 * DllBattle @ 0x39c0, size 0x28 -- if (call(this)) this->+4->+0x14 = 1; return 1. Masked match 10/10.
 * Note: an explicit `int one = 1; b->f = one;` temp is what makes armcc load the pointer
 * before the constant (direct `b->f = 1` swaps r0/r1: 7/10).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x39c0 0x28 xy-decomp/armcc-matching/matched/DllBattle/0x39c0.c
 */
extern int fa(void *);
typedef struct B { char pad[0x14]; unsigned char f; } B;
typedef struct A { int x; B *b; } A;
int fn(A *self) {
    if (fa(self) != 0) {
        B *b = self->b;
        int one = 1;
        b->f = one;
    }
    return 1;
}
