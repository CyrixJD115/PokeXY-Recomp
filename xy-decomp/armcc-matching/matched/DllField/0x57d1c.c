/* DllField function @ CRO file offset 0x57d1c, size 0x18 -- MATCHED (masked, 6/6).
 * range predicate: returns (this->field4 - 4) <= 3 unsigned, i.e. field4 in 4..7.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x57d1c 0x18 xy-decomp/armcc-matching/matched/DllField/0x57d1c.c
 */

struct S { char pad[4]; unsigned v; };

int in_range(struct S *s)
{
    return s->v - 4 <= 3;
}
