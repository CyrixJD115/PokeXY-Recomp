/*
 * DllBattle @ 0x4c374, size 0x3c -- range-checked flag getter: o->p (at +28) holds an int
 * array of 11 slots at +628; returns slot[i] != 0. Out-of-range i calls the assert stub
 * fnA(0,0,0) and returns 0 (sibling of the 0x4af00 / 0x4bb2c range-check idiom).
 * Masked match 15/15.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c374 0x3c xy-decomp/armcc-matching/matched/DllBattle/0x4c374.c
 */
extern void fnA(int a, int b, int c);
struct P { char pad[628]; int v[11]; };
struct O { char pad[28]; struct P *p; };
int fn(struct O *o, unsigned i) {
    struct P *p = o->p;
    if (i < 11)
        return p->v[i] != 0;
    fnA(0, 0, 0);
    return 0;
}
