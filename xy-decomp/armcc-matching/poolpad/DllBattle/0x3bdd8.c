/*
 * DllBattle @ 0x3bdd8, size 0x30 -- trace-prefix store of constant 0xdf9 into c->v (+0x6c, c = arg3), then
 * calls f1(o->p190 + 0x280, 0xdf9) and returns 1.
 * POOL-PAD match (all 10 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: natural -- (`k` is also the value passed on to the callee, so the local is ordinary; the literal-inlined form does not match)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3bdd8 0x30 xy-decomp/armcc-matching/poolpad/DllBattle/0x3bdd8.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct O { char pad[0x190]; char *p; } O;
extern void f1(char *, int);
int fn(O *o, int unused, C *c) {
    int k = 0xdf9;
    if (c->v != k) c->v = k;
    f1(o->p + 0x280, k);
    return 1;
}
