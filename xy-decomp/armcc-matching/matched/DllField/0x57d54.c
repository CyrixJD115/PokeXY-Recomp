/* DllField function @ CRO file offset 0x57d54, size 0x14 -- MATCHED (masked, 5/5).
 * predicate: returns this->field4 == 7.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x57d54 0x14 xy-decomp/armcc-matching/matched/DllField/0x57d54.c
 */

struct S { char pad[4]; unsigned v; };

int is7(struct S *s)
{
    return s->v == 7;
}
