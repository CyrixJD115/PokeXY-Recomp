/*
 * DllBattle @ 0x7a780, size 0x30 -- static-style helper (this in 2nd arg): if (!this->vt[0x42](this)) tail-call fn(this->+0x190) (void).
 * Masked match 12/12.
 * Style: trick(extra-arg) -- unused first arg only so `this` arrives in r1 as in retail
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x7a780 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x7a780.c
 */
typedef struct O { int (**vt)(struct O *); char pad[0x18c]; int x; } O;
extern void fa(int);
void fn(int unused, O *o) {
    if (!o->vt[66](o)) fa(o->x);
}
