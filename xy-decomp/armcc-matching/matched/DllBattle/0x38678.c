/*
 * DllBattle @ 0x38678, size 0x1c -- getter: a = o->a (at +12); b = a ? a->b (at +56) : fnA(a)
 * (fallback lookup with the null pointer); returns b->v (at +28).
 * Masked match 7/7.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x38678 0x1c xy-decomp/armcc-matching/matched/DllBattle/0x38678.c
 */
struct B { char pad[28]; int v; };
struct A { char pad[56]; struct B *b; };
struct O { char pad[12]; struct A *a; };
extern struct B *fnA(struct A *a);
int fn(struct O *o) {
    struct A *a = o->a;
    struct B *b = a ? a->b : fnA(a);
    return b->v;
}
