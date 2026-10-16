/* DllStrInput function @ CRO file offset 0x6148, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * First specimen from a third module, confirming the trivial-stub cluster
 * recurs near the tail of every Dll module built on the shared dialog-proc
 * base class, not just DllDialogCommon/DllNumberInput.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStrInput 0x6148 0x8 xy-decomp/armcc-matching/matched/DllStrInput/0x6148.c
 */

int stub_true(void *this)
{
    return 1;
}
