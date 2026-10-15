/* DllTitle function @ CRO file offset 0xd778, size 0xc -- MASKED MATCH 3/3.
 * Third matched function from this module. Trivial int field setter at
 * this+44 (the same field 0xd770 reads), returns 1 unconditionally.
 * Sits three instructions after 0xd770 in the same dense getter/setter
 * cluster (0xd530-0xd8ac).
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTitle 0xd778 0xc \
 *     xy-decomp/armcc-matching/matched/DllTitle/0xd778.c
 */

int set_field_0x2c(void *this, int v)
{
    *(int *)((char *)this + 44) = v;
    return 1;
}
