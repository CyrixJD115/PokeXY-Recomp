/*
 * DllBattle @ 0x3d21c, size 0x14 -- boolean getter: returns this->+7 (u8) != 1.
 * Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3d21c 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x3d21c.c
 */
typedef struct O { char pad[7]; unsigned char v; } O;
int fn(O *o) { if (o->v == 1) return 0; return 1; }
