/*
 * DllBattle @ 0x4400c, size 0x54 -- sibling of 0x43aec with constant 0x882: trace-prefix store into c->v (+0x6c,
 * c = arg3), then on p = o->p190: virtual slot 4 (vtable+16) of p->q88 with arg 0, null p->q88, clear byte p+0x93,
 * call f1(), tail-call f2(p->r74, 1).
 * POOL-PAD match (all 19 code words identical, same single pool constant): retail starts at 4 mod 8, so one 0
 * alignment word precedes the pool constant and the ldr displacement is 4 larger than a standalone compile's.
 * Style: trick(redundant-local) -- the constant is held in a named local `k` (`int k = 0x882; if (c->v != k) c->v = k;`); the literal-twice form `if (c->v != 0x882) c->v = 0x882;` does not match (checked 2026-10-16). Plausibly a trace macro's temp, true source unknown
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4400c 0x54 xy-decomp/armcc-matching/poolpad/DllBattle/0x4400c.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
typedef struct Q { void (**vt)(struct Q *, int); } Q;
typedef struct P { char pad[0x74]; void *r; char pad2[0x10]; Q *q; char pad3[7]; unsigned char b; } P;
typedef struct O { char pad[0x190]; P *p; } O;
extern void f1(void);
extern void f2(void *, int);
void fn(O *o, int unused, C *c) {
    int k = 0x882;
    P *p;
    Q *q;
    if (c->v != k) c->v = k;
    p = o->p;
    q = p->q;
    q->vt[4](q, 0);
    p->q = 0;
    p->b = 0;
    f1();
    f2(p->r, 1);
}
