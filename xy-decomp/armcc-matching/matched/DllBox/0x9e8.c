/* DllBox function @ CRO file offset 0x9e8, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Right after the import-veneer table (0x180-0x9e7): three identical
 * `return 1` stubs at 0x9e8/0x9f0/0x9f8 (only 0x9e8 claimed here), then
 * a run of bare `bx lr` single-word stubs (0xa00/0xa08/0xa0c/0xa10,
 * weak-evidence class) interleaved with one more `return 1` stub at
 * 0xa04, then the module's first real function (a `push {r3-fp,lr}`
 * dispatcher) at 0xa14. Same recurring tail-of-import-veneer-table
 * leaf-stub cluster shape documented for every other Dll module mined
 * so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x9e8 0x8 xy-decomp/armcc-matching/matched/DllBox/0x9e8.c
 */

int stub_true(void *this)
{
    return 1;
}
