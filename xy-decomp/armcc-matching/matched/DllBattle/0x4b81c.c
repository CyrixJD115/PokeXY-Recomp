/*
 * DllBattle @ 0x4b81c, size 0x10 -- MATCHED (masked, push-less leaf).
 * getter: loads this->+4 -> +16 (object pointer chain) then returns the u8 at +16 (3-deep ldr/ldr/ldrb chain). Family: 0x4bf14 (u8 at +1), 0x544ec (u8 at +0), identical chain.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4b81c 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x4b81c.c
 */
struct B { char pad[16]; unsigned char v; };
struct A { char pad[16]; struct B *b; };
struct O { char pad[4]; struct A *a; };
unsigned char fn(struct O *o) { return o->a->b->v; }
