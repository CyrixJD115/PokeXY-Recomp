/* DllField function @ CRO file offset 0x7f60c, size 0x10 -- MATCHED (masked, 4/4).
 * Table getter: returns the u16 at byte +10 of 12-byte entry [i] of the table passed as 4th arg
 * (r3; r0 = index).
 * Family: same body with a different final ldrh offset: 0x7f628 (+22), 0x7f65c (+8), 0x7f678 (+20)
 * (the +20/+22 ones imply a base offset or a second table half; not yet tried).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x7f60c 0x10 xy-decomp/armcc-matching/matched/DllField/0x7f60c.c
 */

struct E { short a, b, c, d, e; unsigned short k; };

unsigned short get(int i, int x, int y, struct E *t)
{
    return t[i].k;
}
