/*
 * DllBattle @ 0x2c9f4, size 0x34 -- virtual-gated tail call returning int: if (vt[0x98](this)) return fn(this->+0x12c); else return 1.
 * Masked match 13/13.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2c9f4 0x34 xy-decomp/armcc-matching/matched/DllBattle/0x2c9f4.c
 */
typedef struct O { int (**vt)(struct O *); char pad[0x128]; int x; } O;
extern int fa(int);
int fn(O *o) {
    if (o->vt[38](o)) return fa(o->x);
    return 1;
}
