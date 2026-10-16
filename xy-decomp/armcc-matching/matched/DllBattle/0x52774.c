/*
 * DllBattle @ 0x52774, size 0x40 -- range-pair getter: asserts (fnA(0,0,0)) when the u16 slot at +278+2i is zero, then returns byte(+281+2i) - byte(+280+2i) (hi minus lo of the next slot).
 * Masked match 16/16.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x52774 0x40 xy-decomp/armcc-matching/matched/DllBattle/0x52774.c
 */
extern void fnA(int a, int b, int c);
struct O { char pad[278]; unsigned short v[100]; };
unsigned char fn(struct O *o, int i) {
    if (o->v[i] == 0)
        fnA(0, 0, 0);
    return ((unsigned char *)o)[281 + 2*i] - ((unsigned char *)o)[280 + 2*i];
}
