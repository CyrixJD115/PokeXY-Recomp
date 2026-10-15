/* DllField function @ CRO file offset 0x412d0, size 0xc -- MATCHED (masked, 3/3).
 * setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm).
 * Family: same shape as 0x76ec (matched); unmatched siblings of identical bytes at
 * 0x1959c 0x41f24 0x42f7c 0x444f0 0x4344c 0x9888 (+28), 0x417f8 (+44).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x412d0 0xc xy-decomp/armcc-matching/matched/DllField/0x412d0.c
 */

struct S { char pad[28]; int a; int b; };

void set_pair(struct S *s, int a, int b)
{
    s->a = a;
    s->b = b;
}
