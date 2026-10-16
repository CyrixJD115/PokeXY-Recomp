/*
 * DllBattle @ 0x3357c, size 0x14 -- splat setter: this->+0x40/+0x44/+0x48 = v (float); this->+0x4c (u8) = flag (3rd arg).
 * Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3357c 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x3357c.c
 */
typedef struct O { char pad[64]; float x, y, z; unsigned char f; } O;
void fn(O *o, float v, int u, int f) { o->x = v; o->y = v; o->z = v; o->f = f; }
