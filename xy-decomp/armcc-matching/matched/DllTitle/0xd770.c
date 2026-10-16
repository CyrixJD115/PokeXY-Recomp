/* DllTitle function @ CRO file offset 0xd770, size 0x8 -- MASKED MATCH 2/2.
 * First matched function from this module (seventieth-ish module mined by
 * the swarm). Trivial prologue-less int field getter: returns the int at
 * this+44, no calls. Sits in a run of small accessor/getter functions right
 * after the module's large import-veneer table and its first few real
 * functions.
 * Style: natural
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTitle 0xd770 0x8 \
 *     xy-decomp/armcc-matching/matched/DllTitle/0xd770.c
 */

int get_field_0x2c(void *this)
{
    return *(int *)((char *)this + 44);
}
