/* DllField function @ CRO file offset 0x30734, size 0x10 -- MATCHED (masked, 4/4).
 * Leaf field initializer/setter: stores -1 to two consecutive ints at +0,+4.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x30734 0x10 xy-decomp/armcc-matching/matched/DllField/0x30734.c
 */

typedef struct { int a, b; } S;

void init_invalid(S *s)
{
    s->a = -1;
    s->b = -1;
}
