/* DllPhotoViewer function @ CRO file offset 0x19a8, size 0x8 — MATCHED (masked, 2/2).
 * Byte setter: this->+0xd9 = arg (strb r1,[r0,#217]; bx lr).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPhotoViewer 0x19a8 0x8 xy-decomp/armcc-matching/matched/DllPhotoViewer/0x19a8.c
 */

void set_byte_d9(unsigned char *this, int v)
{
    this[0xd9] = (unsigned char)v;
}
