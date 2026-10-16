/*
 * DllBattle @ 0x4b8d4, size 0xc -- MATCHED (masked, push-less leaf).
 * getter: returns the u8 at +9 of the object pointed to by this->+28 (ldr; ldrb). Family: 0xc7580 (+16 -> +72).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4b8d4 0xc xy-decomp/armcc-matching/matched/DllBattle/0x4b8d4.c
 */
struct B { char pad[9]; unsigned char v; };
struct O { char pad[28]; struct B *b; };
unsigned char fn(struct O *o) { return o->b->v; }
