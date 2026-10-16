/*
 * DllBattle @ 0x80820, size 0x38 -- script-command handler: reads one int argument; if the indexed slot (index = ctx3->+12) of the pointer table at +1044 is non-null, passes the argument and slot->+8 to a worker.
 * Masked match 14/14. Pointer-table-by-script-index family (0x80858, 0x82a7c share the head).
 * Style: trick(redundant-local) -- the copies `q = o` and `x = v` exist only to fix armcc's r4/r5 register assignment and the [sp]-before-member load order; without both the output is 11/14.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x80820 0x38 xy-decomp/armcc-matching/matched/DllBattle/0x80820.c
 */
extern void rd(int, int*, int);
extern void fnB(int, int);
typedef struct P { char pad[8]; int m8; } P;
typedef struct O { char pad[1044]; P *t[1]; } O;
typedef struct C { char pad[12]; int i; } C;
void fn(int a, O *o, C *c) {
    int v;
    O *q = o;
    int i = c->i;
    P *p;
    rd(a, &v, 1);
    p = q->t[i];
    if (p) {
        int x = v;
        fnB(p->m8, x);
    }
}
