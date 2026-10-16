/*
 * DllBattle @ 0x797b0, size 0x1c -- MATCHED (masked, push-less leaf).
 * bit-pack builder: (b<<3 & 0x1f8) | 4 | (a<<9 & 0x7e00) -- two fields plus constant tag 4. Retail hoists both mask constants before the shifts/ands; plain expression loads them interleaved.
 * Style: trick(redundant-local) -- named mask locals m1/m2 force armcc to materialise both constants up front (retail order), and the lone `v |= 4` split keeps the tag orr before the final or
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x797b0 0x1c xy-decomp/armcc-matching/matched/DllBattle/0x797b0.c
 */
unsigned fn(unsigned a, unsigned b)
{
    unsigned m1 = 0x1f8, m2 = 0x7e00;
    unsigned v = (b << 3) & m1;
    v |= 4;
    return v | ((a << 9) & m2);
}
