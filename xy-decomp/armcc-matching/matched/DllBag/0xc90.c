/* DllBag function @ CRO file offset 0xc90, size 0x8 -- MATCHED (masked, 2/2).
 * Virtual-override stub: returns 1 (true).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0xc90 0x8 xy-decomp/armcc-matching/matched/DllBag/0xc90.c
 */

int stub_true(void *this)
{
    return 1;
}
