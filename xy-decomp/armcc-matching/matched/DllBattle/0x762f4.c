/*
 * DllBattle @ 0x762f4, size 0x14 -- boolean getter: returns this->+0x694 (u8) == 3.
 * Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x762f4 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x762f4.c
 */
typedef struct O { char pad[1684]; unsigned char v; } O;
int fn(O *o) { return o->v == 3; }
