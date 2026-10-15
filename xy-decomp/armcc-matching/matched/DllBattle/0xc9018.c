/*
 * DllBattle @ 0xc9018, size 0x2c -- forward int arg to vt[68](this,a), tail-call fa(this->+0x14, result) (void).
 * Masked match 11/11. The forwarded int arg (r1 passthrough) is what moves the vtable load into r2.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9018 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xc9018.c
 */
typedef struct O { int (**vt)(struct O *, int); int p[4]; int x; } O;
extern void fa(int, int);
void fn(O *o, int a) { fa(o->x, o->vt[68](o, a)); }
