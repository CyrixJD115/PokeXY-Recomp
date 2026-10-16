/* DllField function @ CRO file offset 0x4a408, size 0x18 -- MATCHED (masked, 6/6).
 * range predicate: returns (this->field4 - 6) <= 3 unsigned, i.e. field4 in 6..9.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x4a408 0x18 xy-decomp/armcc-matching/matched/DllField/0x4a408.c
 */

struct S { char pad[4]; unsigned v; };

int in_range(struct S *s)
{
    return s->v - 6 <= 3;
}
