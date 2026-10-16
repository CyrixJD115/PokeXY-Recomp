/* DllField function @ CRO file offset 0xa0024, size 0xc -- MATCHED (masked, 3/3).
 * getter: returns this->inner(+68)->field(+160).
 * Family: `ldr r0,[r0,#68]; ldr r0,[r0,#N]`; siblings 0xa0bdc (N=152), 0xa0024 (ldrb N=160).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xa0024 0xc xy-decomp/armcc-matching/matched/DllField/0xa0024.c
 */

struct In { char pad[160]; unsigned char v; };
struct S { char pad[68]; struct In *in; };

unsigned char get(struct S *s)
{
    return s->in->v;
}
