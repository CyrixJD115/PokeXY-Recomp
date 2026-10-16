/* DllPssCharaMenu function @ CRO file offset 0x5e0, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Right after the import-veneer table (0x180-0x5df): an identical twin
 * `return 1` stub follows immediately at 0x5e8 (unclaimed here), then two
 * bare `bx lr` single-word stubs (0x5f0/0x5f4, weak-evidence class), then
 * the module's first real function (a `push {r4,r5,r6,r7,r8,lr}`
 * dispatcher) at 0x5f8. Same recurring tail-of-import-veneer-table
 * leaf-stub cluster shape documented for every other Dll module mined so
 * far (see xy-decomp/armcc-matching/README.md "What reliably works" #1).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPssCharaMenu 0x5e0 0x8 xy-decomp/armcc-matching/matched/DllPssCharaMenu/0x5e0.c
 */

int stub_true(void *this)
{
    return 1;
}
