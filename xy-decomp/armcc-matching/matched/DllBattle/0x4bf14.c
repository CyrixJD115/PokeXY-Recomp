/*
 * DllBattle @ 0x4bf14, size 0x10 -- MATCHED (masked, push-less leaf).
 * getter: this->+4->+16 pointer chain, returns the u8 at +1 of the final object. Sibling of 0x4b81c (field +16) and 0x544ec (field +0).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4bf14 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x4bf14.c
 */
struct B { unsigned char a; unsigned char v; };
struct A { char pad[16]; struct B *b; };
struct O { char pad[4]; struct A *a; };
unsigned char fn(struct O *o) { return o->a->b->v; }
