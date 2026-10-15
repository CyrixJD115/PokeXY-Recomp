/* DllVSDemo function @ CRO file offset 0x2b28, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Preceded by a destructor-stage helper ending at 0x2b20 plus its own
 * one-word literal pool at 0x2b24. Same recurring stub shape as
 * 0x3e0/0x1a60/0x1b00 in this module, this time `return 0` instead of
 * `return 1`.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllVSDemo 0x2b28 0x8 xy-decomp/armcc-matching/matched/DllVSDemo/0x2b28.c
 */

int stub_false(void *this)
{
    return 0;
}
