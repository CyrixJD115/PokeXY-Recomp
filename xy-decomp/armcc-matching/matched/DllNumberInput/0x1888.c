/* DllNumberInput function @ CRO file offset 0x1888, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * First module other than DllDialogCommon to be fingerprinted (2026-10-15):
 * confirms the trivial-stub pattern generalizes across Dll modules that
 * share the same dialog-proc base class, not just DllDialogCommon.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllNumberInput 0x1888 0x8 xy-decomp/armcc-matching/matched/DllNumberInput/0x1888.c
 */

int stub_true(void *this)
{
    return 1;
}
