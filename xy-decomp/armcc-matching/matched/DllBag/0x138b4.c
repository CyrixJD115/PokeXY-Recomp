/* DllBag function @ CRO file offset 0x138b4, size 0x8 -- MATCHED (masked, 2/2).
 * Setter: this->byte@0xd9 = v (strb r1,[r0,#217]).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x138b4 0x8 xy-decomp/armcc-matching/matched/DllBag/0x138b4.c
 */

void set_byte_217(unsigned char *this, int v)
{
    this[217] = (unsigned char)v;
}
