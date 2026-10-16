/* DllField function @ CRO file offset 0x3424c, size 0x10 -- MATCHED (masked, 4/4).
 * init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+36.
 * Family: same shape as 0x36d18 (matched).
 * Style: natural -- plain two-field init
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x3424c 0x10 xy-decomp/armcc-matching/matched/DllField/0x3424c.c
 */

struct Owner { char pad[408]; int val; };
struct S { char pad[24]; struct Owner *owner; char pad2[8]; int v; };

void init(struct S *s, struct Owner *o)
{
    s->owner = o;
    s->v = o->val;
}
