/* DllField function @ CRO file offset 0x9ff58, size 0xc -- MATCHED (masked, 3/3).
 * getter: returns this->inner(+68)->field(+148).
 * Family: `ldr r0,[r0,#68]; ldr r0,[r0,#N]`; siblings 0xa0bdc (N=152), 0xa0024 (ldrb N=160).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x9ff58 0xc xy-decomp/armcc-matching/matched/DllField/0x9ff58.c
 */

struct In { char pad[148]; int v; };
struct S { char pad[68]; struct In *in; };

int get(struct S *s)
{
    return s->in->v;
}
