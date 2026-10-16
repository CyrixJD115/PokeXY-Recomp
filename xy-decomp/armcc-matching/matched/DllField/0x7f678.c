/* DllField function @ CRO file offset 0x7f678, size 0x10 -- MATCHED (masked, 4/4).
 * Table getter: returns the u16 at byte +20 of 12-byte entry [i] of the table passed as 4th arg
 * (r3; r0 = index).
 * Entry stride 12 bytes (r0*3<<2); field at +20 from the entry start (>=12 for +20/+22, i.e. reaches into the following entry).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x7f678 0x10 xy-decomp/armcc-matching/matched/DllField/0x7f678.c
 */

struct E { char pad[12]; };

unsigned short get(int i, int x, int y, struct E *t)
{
    return *(unsigned short *)((char *)&t[i] + 20);
}
