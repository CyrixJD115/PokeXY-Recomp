/* DllField function @ CRO file offset 0x297d4, size 0x10 -- MATCHED (masked, 4/4).
 * Leaf field initializer/setter: zero-init: byte at +0 and word at +4.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x297d4 0x10 xy-decomp/armcc-matching/matched/DllField/0x297d4.c
 */

typedef struct { unsigned char a; unsigned char pad[3]; int b; } S;

void init_zero(S *s)
{
    s->a = 0;
    s->b = 0;
}
