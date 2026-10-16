/* DllDialogCommon function @ CRO file offset 0xd18, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * Second of the pair at 0xd10-0xd1f; see 0xd10.c for context. Immediately
 * followed by the message-dispatch function @0xd20 (open item — see
 * docs/decomp/armcc-toolchain.md, block-layout puzzle not yet solved).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDialogCommon 0xd18 0x8 xy-decomp/armcc-matching/matched/DllDialogCommon/0xd18.c
 */

int stub_true(void *this)
{
    return 1;
}
