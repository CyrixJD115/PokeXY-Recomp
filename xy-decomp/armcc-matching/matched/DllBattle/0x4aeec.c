/* DllBattle function @ CRO file offset 0x4aeec, size 0xc -- MATCHED (masked, 3/3).
 * Setter: zeroes the 32-bit field at this+0xc.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x4aeec 0xc xy-decomp/armcc-matching/matched/DllBattle/0x4aeec.c
 */

struct S { int pad[3]; int f; };

void set_field_zero(struct S *s)
{
    s->f = 0;
}
