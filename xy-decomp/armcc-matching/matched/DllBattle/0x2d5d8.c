/*
 * DllBattle @ 0x2d5d8, size 0x44 -- virtual-gated tail call with float arg: if (vt[0x98](this)) fn(this->+0x12c, a) (void); same shape as 0x2ca28.
 * Masked match 17/17.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2d5d8 0x44 xy-decomp/armcc-matching/matched/DllBattle/0x2d5d8.c
 */
typedef struct O { int (**vt)(struct O *); char pad[0x128]; int x; } O;
extern void fa(int, float);
void fn(O *o, float a) {
    if (o->vt[38](o)) fa(o->x, a);
}
