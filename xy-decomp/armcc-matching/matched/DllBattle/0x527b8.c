/*
 * DllBattle @ 0x527b8, size 0x34 -- checked byte getter: asserts (fnA(0,0,0)) when the u16 slot at +278+2i is zero, then returns the u8 at +280+2i (sibling of 0x52774).
 * Masked match 13/13.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x527b8 0x34 xy-decomp/armcc-matching/matched/DllBattle/0x527b8.c
 */
extern void fnA(int a, int b, int c);
struct O { char pad[278]; unsigned short v[100]; };
unsigned char fn(struct O *o, int i) {
    if (o->v[i] == 0)
        fnA(0, 0, 0);
    return *((unsigned char *)&o->v[i] + 2);
}
