/* DllSequence function @ CRO file offset 0x6cc4, size 0x30 — MATCHED (masked, 12/12).
 * Style: natural
 * Two back-to-back calls to the same import veneer on this+0x2c and this+0x30
 * (second one a tail call), second arg `ctx` carried across the first call.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0x6cc4 0x30 xy-decomp/armcc-matching/matched/DllSequence/0x6cc4.c
 */
extern void fnB(void *ctx, void *p, int n);

void Release2_6cc4(unsigned char *this, void *ctx)
{
    fnB(ctx, this + 0x2c, 1);
    fnB(ctx, this + 0x50, 1);
}
