/*
 * DllBattle @ 0x71208, size 0x48 -- trace-prefix store of constant 0xa33 into c->v (+0x6c, c = arg2), then
 * if (f1(o->p190, o->q764)) { f2(o->p190 + 0x140); return 1; } return 0.
 * POOL-PAD match (all 16 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x71208 0x48 xy-decomp/armcc-matching/poolpad/DllBattle/0x71208.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct O { char pad[0x190]; char *p; char pad2[0x5d0]; void *q; } O;
extern int f1(char *, void *);
extern void f2(char *);
int fn(O *o, C *c) {
    int k = 0xa33;
    if (c->v != k) c->v = k;
    if (f1(o->p, o->q)) {
        f2(o->p + 0x140);
        return 1;
    }
    return 0;
}
