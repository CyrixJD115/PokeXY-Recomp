/*
 * DllBattle @ 0x4344c, size 0x2c -- vt[0x80](this,a) result compared: returns r==103||r==104 (written as early-return).
 * Masked match 11/11. The forwarded int arg (r1 passthrough) moves the vtable load into r2. The `||` spelling
 * compiles to cmp/cmpne/moveq/movne (8 words); retail has the branchy beq/moveq/beq shape, which the
 * `if (r == 103) return 0; return r == 104;` spelling reproduces.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4344c 0x2c xy-decomp/armcc-matching/matched/DllBattle/0x4344c.c
 */
typedef struct O { int (**vt)(struct O *, int); } O;
int fn(O *o, int a)
{
    int r = o->vt[32](o, a);
    if (r == 103)
        return 0;
    return r == 104;
}
