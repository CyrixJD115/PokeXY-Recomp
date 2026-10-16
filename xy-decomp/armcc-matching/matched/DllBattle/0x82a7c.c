/*
 * DllBattle @ 0x82a7c, size 0x60 -- script-command handler: reads an int and a float; if the indexed slot of the pointer table at +1044 is non-null, calls a worker with (slot->+8, int, (float)(unsigned)float) -- the float is truncated to an unsigned integer value and converted back.
 * Masked match 24/24. Sibling of 0x80820.
 * Style: trick(redundant-local) -- `q = o` and `x = v` copies only to fix register assignment and the [sp]-before-member load order (22/24 without x).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x82a7c 0x60 xy-decomp/armcc-matching/matched/DllBattle/0x82a7c.c
 */
extern void rd(int, int*, int);
extern void rf(int, float*, int);
extern void fq(int, int, float);
typedef struct P { char pad[8]; int m8; } P;
typedef struct O { char pad[1044]; P *t[1]; } O;
typedef struct C { char pad[12]; int i; } C;
void fn(int a, O *o, C *c) {
    int v;
    float f;
    O *q = o;
    int i = c->i;
    P *p;
    rd(a, &v, 1);
    rf(a, &f, 1);
    p = q->t[i];
    if (p) {
        int x = v;
        fq(p->m8, x, (float)(unsigned)f);
    }
}
