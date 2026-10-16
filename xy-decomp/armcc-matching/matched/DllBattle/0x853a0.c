/*
 * DllBattle @ 0x853a0, size 0x5c -- script-command handler: reads one flag, then for every object in the row (index = ctx3->+12) of the per-row object lists (counts at +804, arrays at +844) calls virtual slot 42 (+0xa8) with (flag != 0).
 * Masked match 23/23. Differs from 0x7bae0 only in the vtable slot.
 * Style: trick(redundant-local) -- `q = o` copy only to fix armcc's register assignment; sibling of 0x7bae0.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x853a0 0x5c xy-decomp/armcc-matching/matched/DllBattle/0x853a0.c
 */
extern void rd(int, int*, int);
struct X;
typedef struct X { int (**vt)(struct X*, int); } X;
typedef struct O { char pad[804]; int n[1]; char pad2[844 - 808]; X **arr[1]; } O;
typedef struct C { char pad[12]; int i; } C;
void fn(int a, O *o, C *c) {
    int v;
    int i = c->i;
    int k;
    int n;
    O *q = o;
    rd(a, &v, 1);
    n = q->n[i];
    for (k = 0; k < n; k++) {
        X *x = q->arr[i][k];
        x->vt[42](x, v != 0);
    }
}
