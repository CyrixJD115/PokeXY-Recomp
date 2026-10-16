/* DllDialogCommon function @ CRO file offset 0x2e0, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * Second of the six-stub run at 0x2d8-0x2ff; see 0x2d8.c for context.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0x2e0 0x8 xy-decomp/armcc-matching/matched/DllDialogCommon/0x2e0.c
 */

int stub_true(void *this)
{
    return 1;
}
