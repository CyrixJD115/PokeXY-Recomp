/* DllPssPlaySelect function @ CRO file offset 0x2c8, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits in the recurring tail-of-import-veneer-table leaf-stub cluster seen
 * in every Dll module mined so far: a run of 24 `ldr pc,[pc,#-4]` import
 * veneers (0x180-0x2c4), then two identical `return 1` stubs at 0x2c8/0x2d0
 * (this file claims 0x2c8), three bare `bx lr` single-word stubs at
 * 0x2d8/0x2dc/0x2e0, then the module's first real function (a notification
 * dispatcher sharing the same shape as DllNumberInput@0x1890) at 0x2e4.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPssPlaySelect 0x2c8 0x8 xy-decomp/armcc-matching/matched/DllPssPlaySelect/0x2c8.c
 */

int stub_true(void *this)
{
    return 1;
}
