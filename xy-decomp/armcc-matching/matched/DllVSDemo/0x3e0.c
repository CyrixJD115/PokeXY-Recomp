/* DllVSDemo function @ CRO file offset 0x3e0, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * First real function right after the module's import-veneer table
 * (0x180-0x3df). One of a run of five near-identical `mov r0,#1; bx lr`
 * stubs (0x3e0/0x3e8/0x3f0/0x3f8/0x400) followed by three bare `bx lr`
 * weak-evidence stubs (0x408/0x40c/0x410), then a real dispatcher at
 * 0x414 — the standard tail-of-import-veneer-table leaf-stub cluster
 * shape documented for every other Dll module mined so far.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllVSDemo 0x3e0 0x8 xy-decomp/armcc-matching/matched/DllVSDemo/0x3e0.c
 */

int stub_true(void *this)
{
    return 1;
}
