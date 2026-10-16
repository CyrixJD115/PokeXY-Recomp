/* DllFieldEventSlopeSlip function @ CRO file offset 0x268, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after the import-veneer table (0x180-0x227, 18 entries) and a
 * real notification/init function (0x228-0x267); immediately followed by
 * another real function at 0x270. Same recurring tail-of-veneer-table
 * leaf-stub shape already documented across every other Dll module mined
 * by this swarm (DllDialogCommon@0x2d8 etc.) — likely a default
 * virtual-method override (IsXxx() == true) on a shared base class.
 *
 * Style: natural -- trivial stub/setter (weak evidence: any function of this shape compiles identically)
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllFieldEventSlopeSlip 0x268 0x8 xy-decomp/armcc-matching/matched/DllFieldEventSlopeSlip/0x268.c
 */

int stub_true(void *this)
{
    return 1;
}
