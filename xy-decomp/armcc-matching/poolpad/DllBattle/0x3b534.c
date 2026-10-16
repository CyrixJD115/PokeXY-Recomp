/*
 * DllBattle @ 0x3b534, size 0x44 -- trace-prefix store of constant 0x1465 into c->v (+0x6c, c = arg3), then
 * releases the object at o->p190->q240: if non-null, virtual call slot 1 (vtable+4) on it and null the field.
 * POOL-PAD match (all 15 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: natural -- (the object is loaded into a local `q` and tested `q != 0`; testing `p->q` twice instead
 *   makes armcc emit `movs r2,r0` + an extra `mov r0,r2`)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3b534 0x44 xy-decomp/armcc-matching/poolpad/DllBattle/0x3b534.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct Q { void (**vt)(struct Q *); } Q;
typedef struct P { char pad[0x240]; Q *q; } P;
typedef struct O { char pad[0x190]; P *p; } O;
void fn(O *o, int unused, C *c) {
    int k = 0x1465;
    P *p;
    Q *q;
    if (c->v != k) c->v = k;
    p = o->p;
    q = p->q;
    if (q != 0) {
        q->vt[1](q);
        p->q = 0;
    }
}
