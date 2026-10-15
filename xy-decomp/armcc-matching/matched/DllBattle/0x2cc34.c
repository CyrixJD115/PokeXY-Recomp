/*
 * DllBattle @ 0x2cc34, size 0xc -- virtual tail call: return this->vt[0x98](this).
 * Masked match 3/3.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2cc34 0xc xy-decomp/armcc-matching/matched/DllBattle/0x2cc34.c
 */
typedef struct O { int (**vt)(struct O *); } O;
int fn(O *o) { return o->vt[38](o); }
