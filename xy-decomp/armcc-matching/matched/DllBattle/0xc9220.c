/*
 * DllBattle @ 0xc9220, size 0x2c -- virtual call then tail-call: fn(this->+0xc, this->vt[0x5c](this)) (void).
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9220 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xc9220.c
 */
typedef struct O { int (**vt)(struct O *); int p[2]; int x; } O;
extern void fa(int, int);
void fn(O *o) { fa(o->x, o->vt[23](o)); }
