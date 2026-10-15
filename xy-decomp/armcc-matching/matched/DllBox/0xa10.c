/* DllBox function @ CRO file offset 0xa10, size 0x4 -- MATCHED (masked, 1/1).
 * Bare bx lr no-op void stub (weak evidence), in the run right before the
 * module first real function at 0xa14.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0xa10 0x4 xy-decomp/armcc-matching/matched/DllBox/0xa10.c
 */

void noop(void *this)
{
}
