/*
 * DllBattle @ 0x544a0, size 0x4c -- range-checked int getter: asserts (fnA(0,0,0)) when i >= 24 and when o->p->v[i] (int array at +132 of the object at o+8) is 0, then returns o->p->v[i].
 * Masked match 19/19.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x544a0 0x4c xy-decomp/armcc-matching/matched/DllBattle/0x544a0.c
 */
extern void fnA(int a, int b, int c);
struct P { char pad[132]; int v[24]; };
struct O { char pad[8]; struct P *p; };
int fn(struct O *o, unsigned i) {
    struct P *p = o->p;
    if (i >= 24)
        fnA(0, 0, 0);
    if (p->v[i] == 0)
        fnA(0, 0, 0);
    return p->v[i];
}
