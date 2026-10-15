/* DllField function @ CRO file offset 0x9ae8, size 0x8 -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x9ae8 0x8 xy-decomp/armcc-matching/matched/DllField/0x9ae8.c
 */

int stub_true(void *this)
{
    return 1;
}
