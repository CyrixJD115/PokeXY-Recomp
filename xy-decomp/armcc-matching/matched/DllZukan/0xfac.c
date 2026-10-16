/* DllZukan function @ CRO file offset 0xfac, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Twin of DllZukan@0xf9c, one of three near-identical `return 1` stubs
 * (0xf9c/0xfa4/0xfac) in the same tail-of-import-veneer-table leaf-stub
 * cluster (0xf98-0xfd7). Immediately followed by two bare `bx lr`
 * weak-evidence stubs at 0xfb4/0xfb8.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0xfac 0x8 xy-decomp/armcc-matching/matched/DllZukan/0xfac.c
 */

int stub_true(void *this)
{
    return 1;
}
