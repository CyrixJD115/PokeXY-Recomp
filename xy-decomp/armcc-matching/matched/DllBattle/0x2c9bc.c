/*
 * DllBattle @ 0x2c9bc, size 0x30 -- virtual-gated tail call: if (this->vt[0x98](this)) tail-call fn(this->+0x12c) (void).
 * Masked match 12/12.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2c9bc 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x2c9bc.c
 */
typedef struct O { int (**vt)(struct O *); char pad[0x128]; int x; } O;
extern void fa(int);
void fn(O *o) {
    if (o->vt[38](o)) fa(o->x);
}
