/* DllTownmap function @ CRO file offset 0x438, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Right after the import-veneer table (0x180-0x437): a run of three
 * near-identical `return 1` stubs (0x438/0x440/0x448, only 0x438
 * claimed here) interleaved with bare `bx lr` single-word stubs
 * (0x450/0x454/0x460/0x464/0x468, weak-evidence class, unclaimed),
 * then the conditional-accessor function at 0x46c (already matched).
 * Same recurring tail-of-import-veneer-table leaf-stub cluster shape
 * documented for every other Dll module mined so far.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x438 0x8 xy-decomp/armcc-matching/matched/DllTownmap/0x438.c
 */

int stub_true(void *this)
{
    return 1;
}
