/*
 * DllBattle @ 0x12a30, size 0x14 -- vec3 setter: this->+52 = (x,y,z) float triple by value; this->+0x4c (u8) = 1 (dirty flag).
 * Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x12a30 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x12a30.c
 */
typedef struct O { char pad[52]; float x, y, z; char pad2[12]; unsigned char f; } O;
void fn(O *o, float x, float y, float z) { o->x = x; o->y = y; o->z = z; o->f = 1; }
