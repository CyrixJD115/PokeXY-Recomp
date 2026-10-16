/* DllDialogCommon function @ CRO file offset 0x2d8, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * One of a run of six near-identical 1-2 word leaf stubs at 0x2d8-0x2ff
 * (this file + 0x2e0.c, 0x2e8.c, 0x2f0.c, 0x2f4.c, 0x2f8.c, 0x2fc.c),
 * immediately preceding the big state-machine function @0x300. Likely
 * default virtual-method overrides on the dialog-proc base class
 * (e.g. IsXxx() == true); no further evidence narrows the real name.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0x2d8 0x8 xy-decomp/armcc-matching/matched/DllDialogCommon/0x2d8.c
 */

int stub_true(void *this)
{
    return 1;
}
