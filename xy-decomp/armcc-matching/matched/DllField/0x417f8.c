/* DllField function @ CRO file offset 0x417f8, size 0xc -- MATCHED (masked, 3/3).
 * setter: stores arg1/arg2 into the adjacent int pair at this+44/this+48 (add r0,#44; stm).
 * Family: same shape as 0x412d0 (matched).
 * Style: natural -- plain two-field setter
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x417f8 0xc xy-decomp/armcc-matching/matched/DllField/0x417f8.c
 */

struct S { char pad[44]; int a; int b; };

void set_pair(struct S *s, int a, int b)
{
    s->a = a;
    s->b = b;
}
