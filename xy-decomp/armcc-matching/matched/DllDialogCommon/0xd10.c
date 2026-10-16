/* DllDialogCommon function @ CRO file offset 0xd10, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 0 (false), ignores `this`.
 * Style: natural
 *
 * First of a pair of trivial stubs at 0xd10-0xd1f, sandwiched between
 * the destructor @0xcdc and the message-dispatch function @0xd20.
 * Likely a default virtual-method override on the dialog-proc base
 * class (e.g. IsXxx() == false).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0xd10 0x8 xy-decomp/armcc-matching/matched/DllDialogCommon/0xd10.c
 */

int stub_false(void *this)
{
    return 0;
}
