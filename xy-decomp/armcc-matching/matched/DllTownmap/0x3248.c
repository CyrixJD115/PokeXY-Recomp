/* DllTownmap function @ CRO file offset 0x3248, size 0xc - MATCHED (masked, 3/3).
 * double-indirection getter: return *this[1] (ldr [r0,#4]; ldr [r0]).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x3248 0xc xy-decomp/armcc-matching/matched/DllTownmap/0x3248.c
 */

int get_deref(int **this)
{
    return *this[1];
}
