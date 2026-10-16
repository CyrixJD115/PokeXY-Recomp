/* DllDialogCommon function @ CRO file offset 0x2f8, size 0x4 — MATCHED (masked, 1/1). Trivial virtual-override stub: single `bx lr`, a true no-op void override.
 * Style: natural
 *
 * Sixth of the six-stub run at 0x2d8-0x2ff; see 0x2d8.c for context (and
 * 0x2f0.c for the weak-evidence caveat shared by all four `bx lr` stubs).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0x2f8 0x4 xy-decomp/armcc-matching/matched/DllDialogCommon/0x2f8.c
 */

void stub_noop(void *this)
{
}
