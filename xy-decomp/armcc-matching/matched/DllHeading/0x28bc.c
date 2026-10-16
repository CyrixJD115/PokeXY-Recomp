/* DllHeading function @ CRO file offset 0x28bc, size 0xc — MATCHED (masked, 3/3,
 * sixth module mined). Trivial field setter: zeroes one byte-sized flag at
 * offset 0xb4 of `this`, void return. Sits between two lone `bx lr` no-op
 * stubs (0x28b0/0x28b4/0x28b8) and the start of the next function's prologue
 * (0x28c8, `push {r4-r9,sl,fp,lr}`) — same cluster shape documented for
 * DllDialogCommon/DllStrInput: default virtual-method overrides and tiny
 * setters pack together near a dialog-proc base class's vtable tail.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHeading 0x28bc 0xc xy-decomp/armcc-matching/matched/DllHeading/0x28bc.c
 * Style: natural
 */

void reset_flag(void *this)
{
    *((unsigned char *)this + 0xb4) = 0;
}
