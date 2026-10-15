/*
 * DllBattle @ 0xc8eb0, size 0x1c -- virtual bool wrapper: return vt[0x48](this) != 0.
 * Masked match 7/7.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc8eb0 0x1c xy-decomp/armcc-matching/matched/DllBattle/0xc8eb0.c
 */
typedef struct O { int (**vt)(struct O *); } O;
int fn(O *o) { return o->vt[18](o) != 0; }
