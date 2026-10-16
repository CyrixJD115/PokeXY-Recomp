/* DllSequence function @ CRO file offset 0xf378, size 0x38 -- MATCHED (masked, 14/14).
 * Style: natural
 * Stores -1 at this+0x34, then two 3-arg veneer calls (this+0x2c, this+0x30).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xf378 0x38 xy-decomp/armcc-matching/matched/DllSequence/0xf378.c
 */
extern void fnA(void *ctx, void *p, int n);

void Release2_f378(unsigned char *this, void *ctx)
{
    *(int *)(this + 0x34) = -1;
    fnA(ctx, this + 0x2c, 1);
    fnA(ctx, this + 0x30, 1);
}
