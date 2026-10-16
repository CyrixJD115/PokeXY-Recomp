/* DllSequence function @ CRO file offset 0xb664, size 0x34 — MATCHED (masked, 13/13).
 * Style: natural
 * Calls veneer fnA on this+0x2c, then tail-calls fnD(ctx, this+0x30, 0x11, 1).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xb664 0x34 xy-decomp/armcc-matching/matched/DllSequence/0xb664.c
 */
extern void fnA(void *ctx, void *p, int n);
extern void fnD(void *ctx, void *p, int a, int b);

void Release2_b664(unsigned char *this, void *ctx)
{
    fnA(ctx, this + 0x2c, 1);
    fnD(ctx, this + 0x30, 0x11, 1);
}
