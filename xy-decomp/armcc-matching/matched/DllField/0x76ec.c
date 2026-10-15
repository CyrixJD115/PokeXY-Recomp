/* DllField function @ CRO file offset 0x76ec, size 0xc -- MATCHED (masked, 3/3).
 * setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x76ec 0xc xy-decomp/armcc-matching/matched/DllField/0x76ec.c
 */

struct S { char pad[28]; int a; int b; };

void set_pair(struct S *s, int a, int b)
{
    s->a = a;
    s->b = b;
}
