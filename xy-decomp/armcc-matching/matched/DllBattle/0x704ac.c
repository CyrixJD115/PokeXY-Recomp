/*
 * DllBattle @ 0x704ac, size 0x20 -- MATCHED (masked, push-less leaf).
 * bit-pack builder + store: *p = (a<<4 & 0x70) | 3 | (b<<7 & 0x380) (two fields, constant tag 3).
 * Style: trick(redundant-local) -- named mask locals m1/m2 hoist both constants (retail puts mov ip,#0x70 / mov r3,#0x380 first) and tag orr split into its own statement
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x704ac 0x20 xy-decomp/armcc-matching/matched/DllBattle/0x704ac.c
 */
void fn(unsigned *p, unsigned a, unsigned b)
{
    unsigned m1 = 0x70, m2 = 0x380;
    unsigned v = (a << 4) & m1;
    v |= 3;
    *p = v | ((b << 7) & m2);
}
