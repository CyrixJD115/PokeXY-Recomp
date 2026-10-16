/* DllBox function @ CRO file offset 0x1dd58, size 0x8 -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1, ignores `this`.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x1dd58 0x8 xy-decomp/armcc-matching/matched/DllBox/0x1dd58.c
 */

int stub_true(void *this)
{
    return 1;
}
