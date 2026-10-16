/* DllField function @ CRO file offset 0x1a448, size 0x14 -- MATCHED (masked, 5/5).
 * Leaf field initializer/setter: zero-style init: stores 0xFFFF (-1) to three consecutive u16 fields at +0,+2,+4 (e.g. invalid-id triple).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x1a448 0x14 xy-decomp/armcc-matching/matched/DllField/0x1a448.c
 */

typedef struct { short a, b, c; } S;

void init_ids_invalid(S *s)
{
    s->a = -1;
    s->b = -1;
    s->c = -1;
}
