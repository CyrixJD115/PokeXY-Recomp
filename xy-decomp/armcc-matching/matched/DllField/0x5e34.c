/* DllField function @ CRO file offset 0x5e34, size 0x1c -- MATCHED (masked, 7/7).
 * zeroes four floats at this+4..+16 (vldr s0 from the pool word, 4x vstr); size includes the literal-pool word.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x5e34 0x1c xy-decomp/armcc-matching/matched/DllField/0x5e34.c
 */

struct S { char pad[4]; float a, b, c, d; };

void zero_floats(struct S *s)
{
    s->a = 0.0f;
    s->b = 0.0f;
    s->c = 0.0f;
    s->d = 0.0f;
}
