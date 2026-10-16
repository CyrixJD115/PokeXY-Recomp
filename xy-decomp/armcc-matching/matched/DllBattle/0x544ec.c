/*
 * DllBattle @ 0x544ec, size 0x10 -- MATCHED (masked, push-less leaf).
 * getter: this->+4->+16 pointer chain, returns the u8 at +0 of the final object. Sibling of 0x4b81c / 0x4bf14.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x544ec 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x544ec.c
 */
struct B { unsigned char v; };
struct A { char pad[16]; struct B *b; };
struct O { char pad[4]; struct A *a; };
unsigned char fn(struct O *o) { return o->a->b->v; }
