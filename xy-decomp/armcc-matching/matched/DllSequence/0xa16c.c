/* DllSequence function @ CRO file offset 0xa16c, size 0x30 — MATCHED (masked, 12/12).
 * Style: natural
 * Two back-to-back calls to the same import veneer on this+0x2c and this+0x30
 * (second one a tail call), second arg `ctx` carried across the first call.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xa16c 0x30 xy-decomp/armcc-matching/matched/DllSequence/0xa16c.c
 */
extern void fnC(void *ctx, void *p, int n);

void Release2_a16c(unsigned char *this, void *ctx)
{
    fnC(ctx, this + 0x2c, 1);
    fnC(ctx, this + 0x30, 1);
}
