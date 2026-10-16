/* DllPssIconSelect function @ CRO file offset 0x540, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after the module's import-veneer table (0x180-0x53f, 120
 * entries): this stub, a twin `return 1` stub at 0x548, then three bare
 * `bx lr` single-word stubs (0x550/0x554/0x558, weak-evidence class,
 * unclaimed), then the module's first real function (a notification/
 * dispatcher-shaped helper with a masked `bl` call) at 0x55c — same
 * recurring tail-of-import-veneer-table leaf-stub cluster shape
 * documented for every other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPssIconSelect 0x540 0x8 xy-decomp/armcc-matching/matched/DllPssIconSelect/0x540.c
 */

int stub_true(void *this)
{
    return 1;
}
