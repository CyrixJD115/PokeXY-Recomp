/* DllVSDemo function @ CRO file offset 0x1b00, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sandwiched between two conditional-free helpers (one ending at 0x1afc,
 * frees a member at +0 if non-null, zeroes it; another starting right
 * after this stub at 0x1b08, same idiom on a member at +0x1c). Same
 * recurring stub shape as 0x3e0 and 0x1a60 in this module.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllVSDemo 0x1b00 0x8 xy-decomp/armcc-matching/matched/DllVSDemo/0x1b00.c
 */

int stub_true(void *this)
{
    return 1;
}
