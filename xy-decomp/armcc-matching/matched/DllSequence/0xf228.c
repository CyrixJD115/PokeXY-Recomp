/* DllSequence function @ CRO file offset 0xf228, size 0x34 -- MATCHED (masked, 13/13).
 * Style: natural
 * Twin of 0xb664: fnA(ctx,this+0x2c,1); tail fnD(ctx,this+0x30,0x11,1).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xf228 0x34 xy-decomp/armcc-matching/matched/DllSequence/0xf228.c
 */
extern void fnA(void *ctx, void *p, int n);
extern void fnD(void *ctx, void *p, int a, int b);

void Release2_f228(unsigned char *this, void *ctx)
{
    fnA(ctx, this + 0x2c, 1);
    fnD(ctx, this + 0x30, 0x11, 1);
}
