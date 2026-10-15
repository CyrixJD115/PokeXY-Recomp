/* DllTownmap function @ CRO file offset 0x488, size 0x10.
 *
 * Shape: calls a local module function (CRO offset 0x1f94) with `this`
 * unchanged, discards its result, unconditionally returns 1. The
 * `push {r4, lr}` is AAPCS stack-alignment padding (r4 is never
 * otherwise touched), same idiom already documented for
 * DllDialogCommon@0x6c4.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x488 0x10 xy-decomp/armcc-matching/matched/DllTownmap/0x488.c
 */

extern void Fn1f94(void *a0);

int func_488(void *a0)
{
    Fn1f94(a0);
    return 1;
}
