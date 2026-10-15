/*
 * DllBattle @ 0x2c8c0, size 0x38 -- virtual-gated tail call returning float: if (vt[0x98](this)) return fn(this->+0x12c); else return 0.0f.
 * Masked match 14/14.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2c8c0 0x38 xy-decomp/armcc-matching/matched/DllBattle/0x2c8c0.c
 */
typedef struct O { int (**vt)(struct O *); char pad[0x128]; int x; } O;
extern float fa(int);
float fn(O *o) {
    if (o->vt[38](o)) return fa(o->x);
    return 0.0f;
}
