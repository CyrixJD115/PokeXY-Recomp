/*
 * DllBattle @ 0xc9298, size 0x2c -- same shape, vt[0x60], callee differs.
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9298 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xc9298.c
 */
typedef struct O { int (**vt)(struct O *); int p[2]; int x; } O;
extern void fa(int, int);
void fn(O *o) { fa(o->x, o->vt[24](o)); }
