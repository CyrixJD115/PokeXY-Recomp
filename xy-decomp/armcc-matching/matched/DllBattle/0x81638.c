/*
 * DllBattle @ 0x81638, size 0x14 -- boolean getter: returns this->+0x10 (u8) == 6.
 * Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x81638 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x81638.c
 */
typedef struct O { char pad[16]; unsigned char v; } O;
int fn(O *o) { return o->v == 6; }
