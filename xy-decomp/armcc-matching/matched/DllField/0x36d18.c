/* DllField function @ CRO file offset 0x36d18, size 0x10 -- MATCHED (masked, 4/4).
 * init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+32.
 * Family: `str r1,[r0,#24]; ldr r1,[r1,#408]; str r1,[r0,#N]`; siblings at 0x2aec4 (N=28),
 * 0x343b4 (N=28), 0x3424c (N=36), 0x387c0 (N=32), 0x3e5c8 (N=32).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x36d18 0x10 xy-decomp/armcc-matching/matched/DllField/0x36d18.c
 */

struct Owner { char pad[408]; int val; };
struct S { char pad[24]; struct Owner *owner; int a; int b; int c; int d; };

void init(struct S *s, struct Owner *o)
{
    s->owner = o;
    s->b = o->val;
}
