/*
 * DllBattle @ 0x43fa8, size 0x60 -- trace-prefix store of constant 0x703 into c->v (+0x6c, c = arg3), zeroes the
 * ints at o+0x754/+0x758, then on p = o->p190: virtual slot 4 (vtable+16) of p->q88 with arg 0, null p->q88,
 * clear byte p+0x93, call f1(0), tail-call f2(p->r74, 1). Sibling of 0x43aec/0x4400c.
 * POOL-PAD match (all 22 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x43fa8 0x60 xy-decomp/armcc-matching/poolpad/DllBattle/0x43fa8.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct Q { void (**vt)(struct Q *, int); } Q;
typedef struct P { char pad[0x74]; void *r; char pad2[0x10]; Q *q; char pad3[7]; unsigned char b; } P;
typedef struct O { char pad[0x190]; P *p; char pad4[0x5c0]; int a754; int a758; } O;
extern void f1(int);
extern void f2(void *, int);
void fn(O *o, int unused, C *c) {
    int k = 0x703;
    P *p;
    Q *q;
    if (c->v != k) c->v = k;
    o->a754 = 0;
    o->a758 = 0;
    p = o->p;
    q = p->q;
    q->vt[4](q, 0);
    p->q = 0;
    p->b = 0;
    f1(0);
    f2(p->r, 1);
}
