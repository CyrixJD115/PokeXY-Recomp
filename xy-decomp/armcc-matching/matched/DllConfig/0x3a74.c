/* DllConfig function @ CRO file offset 0x3a74, size 0x8 — MATCHED (masked, 2/2).
 *
 * Trivial virtual-override stub: unconditional `return 0`. Sits isolated
 * between two destructor-stage helpers (0x3a40-0x3a70 and 0x3a7c onward)
 * in the tail run of the module's .text, the same cluster shape documented
 * for DllDialogCommon/DllNumberInput/DllStrInput (default virtual-method
 * overrides on the shared dialog-proc base class).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllConfig 0x3a74 0x8 xy-decomp/armcc-matching/matched/DllConfig/0x3a74.c
 */

int F(void *this)
{
    return 0;
}
