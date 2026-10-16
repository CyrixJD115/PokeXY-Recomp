/*
 * DllBattle @ 0xc7580, size 0xc -- MATCHED (masked, push-less leaf).
 * getter: returns the u8 at +72 of the object pointed to by this->+16 (ldr; ldrb). Sibling of 0x4b8d4.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc7580 0xc xy-decomp/armcc-matching/matched/DllBattle/0xc7580.c
 */
struct B { char pad[72]; unsigned char v; };
struct O { char pad[16]; struct B *b; };
unsigned char fn(struct O *o) { return o->b->v; }
