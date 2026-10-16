/*
 * DllBattle @ 0x3e7a4, size 0x38 -- trace-prefix store of constant 0x14ab into c->v (+0x6c, c = arg3), then
 * returns (f1(o->p190->q4f4, 0) != 0).
 * POOL-PAD match (all 12 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: trick(redundant-local) -- the constant is held in a named local `k` (`int k = 0x14ab; if (c->v != k) c->v = k;`); the literal-twice form `if (c->v != 0x14ab) c->v = 0x14ab;` does not match (checked 2026-10-16). Plausibly a trace macro's temp, true source unknown
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3e7a4 0x38 xy-decomp/armcc-matching/poolpad/DllBattle/0x3e7a4.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct P { char pad[0x4f4]; void *q; } P;
typedef struct O { char pad[0x190]; P *p; } O;
extern int f1(void *, int);
int fn(O *o, int unused, C *c) {
    int k = 0x14ab;
    if (c->v != k) c->v = k;
    return f1(o->p->q, 0) != 0;
}
