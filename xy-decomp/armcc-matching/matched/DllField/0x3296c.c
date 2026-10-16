/* DllField function @ CRO file offset 0x3296c, size 0x1c -- MATCHED (masked, 7/7).
 * Leaf field initializer/setter: zero-init of 5 fields; note byte at +0x14 is stored BEFORE the word at +0x10 (source order matters).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x3296c 0x1c xy-decomp/armcc-matching/matched/DllField/0x3296c.c
 */

typedef struct { int a, b, c, pad; int d; unsigned char e; } S;

void init_zero(S *s)
{
    s->a = 0;
    s->b = 0;
    s->c = 0;
    s->e = 0;
    s->d = 0;
}
