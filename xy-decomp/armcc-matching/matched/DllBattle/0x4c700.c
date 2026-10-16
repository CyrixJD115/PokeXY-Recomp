/* DllBattle function @ CRO file offset 0x4c700, size 0xc -- MATCHED (masked, 3/3).
 * Setter: stores byte 6 at this+0x10.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x4c700 0xc xy-decomp/armcc-matching/matched/DllBattle/0x4c700.c
 */

struct S { unsigned char pad[16]; unsigned char f; };

void set_field_6(struct S *s)
{
    s->f = 6;
}
