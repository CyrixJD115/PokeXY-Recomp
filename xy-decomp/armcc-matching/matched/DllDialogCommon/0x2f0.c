/* DllDialogCommon function @ CRO file offset 0x2f0, size 0x4 — MATCHED (masked, 1/1). Trivial virtual-override stub: single `bx lr`, a true no-op void override.
 * Style: natural
 *
 * Fourth of the six-stub run at 0x2d8-0x2ff; see 0x2d8.c for context.
 * A lone `bx lr` is consistent with any argument-less void function with
 * no observable side effect, so this is the weakest-evidence match in
 * the set — any no-op body compiles to the same single word.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0x2f0 0x4 xy-decomp/armcc-matching/matched/DllDialogCommon/0x2f0.c
 */

void stub_noop(void *this)
{
}
