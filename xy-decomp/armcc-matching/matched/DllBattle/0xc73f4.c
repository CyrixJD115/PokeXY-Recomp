/*
 * DllBattle @ 0xc73f4, size 0x10 -- indexed getter: returns this->+0xb1c[arg & 1] (int array).
 * Masked match 4/4.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc73f4 0x10 xy-decomp/armcc-matching/matched/DllBattle/0xc73f4.c
 */
typedef struct O { char pad[2844]; int v[2]; } O;
int fn(O *o, int i) { return o->v[i & 1]; }
