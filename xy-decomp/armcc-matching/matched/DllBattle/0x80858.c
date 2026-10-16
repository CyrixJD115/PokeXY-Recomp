/*
 * DllBattle @ 0x80858, size 0x3c -- script-command handler: reads one argument (float-reader veneer, stored as a raw word); if the indexed slot of the pointer table at +1044 is non-null and its +12 field is non-zero, stores the argument into slot->+116.
 * Masked match 15/15. Sibling of 0x80820.
 * Style: trick(redundant-local) -- the copies `q = o` and `x = v` exist only to fix armcc's register assignment (r4/r5) and conditional-execution shape.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x80858 0x3c xy-decomp/armcc-matching/matched/DllBattle/0x80858.c
 */
extern void rf(int, int*, int);
typedef struct P { char pad[12]; int m12; char pad2[116 - 16]; int m116; } P;
typedef struct O { char pad[1044]; P *t[1]; } O;
typedef struct C { char pad[12]; int i; } C;
void fn(int a, O *o, C *c) {
    int v;
    O *q = o;
    int i = c->i;
    P *p;
    rf(a, &v, 1);
    p = q->t[i];
    if (p) {
        int x = v;
        if (p->m12) p->m116 = x;
    }
}
