/* DllTitle function @ CRO file offset 0xd7a8, size 0x8 -- MASKED MATCH 2/2.
 * Second matched function from this module. Same shape as 0xd770 in the
 * same module: a trivial prologue-less int field getter, this time at
 * this+12, no calls. Part of a dense cluster of small getter/setter/
 * predicate functions between 0xd530 and 0xd8ac in this module's .text.
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTitle 0xd7a8 0x8 \
 *     xy-decomp/armcc-matching/matched/DllTitle/0xd7a8.c
 */

int get_field_0xc(void *this)
{
    return *(int *)((char *)this + 12);
}
