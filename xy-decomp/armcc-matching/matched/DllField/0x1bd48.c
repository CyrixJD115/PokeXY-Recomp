/* DllField function @ CRO file offset 0x1bd48, size 0x14 -- MATCHED (masked, 5/5).
 * Leaf field initializer/setter: stores arg2 at +0x18, and arg2->[+0x50]->[+0x30] at +0x20.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x1bd48 0x14 xy-decomp/armcc-matching/matched/DllField/0x1bd48.c
 */

typedef struct { unsigned char pad[24]; int a; unsigned char pad2[4]; int b; } S;
typedef struct { unsigned char pad[80]; char *p; } Src;

void setup(S *s, Src *r)
{
    s->a = (int)r;
    s->b = *(int *)(r->p + 48);
}
