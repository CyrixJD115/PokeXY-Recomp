/* DllUSPassage function @ CRO file offset 0x750, size 0x8 — MATCHED (masked, 2/2). Trivial
 * field getter: returns the int field at `this+0xb4`, no branches/calls.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPassage 0x750 0x8 xy-decomp/armcc-matching/matched/DllUSPassage/0x750.c
 */

int field_getter(void *this)
{
    return *(int *)((char *)this + 0xb4);
}
