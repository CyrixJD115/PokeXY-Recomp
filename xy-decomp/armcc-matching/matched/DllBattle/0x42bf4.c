/*
 * DllBattle @ 0x42bf4, size 0x18 -- virtual accessor-then-read: p = this->vt[0x3e](this); return p->byte(+0x27).
 * Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x42bf4 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x42bf4.c
 */
typedef struct P { char pad[0x27]; unsigned char b; } P;
typedef struct O { P *(**vt)(struct O *); } O;
unsigned char fn(O *o) { return o->vt[62](o)->b; }
