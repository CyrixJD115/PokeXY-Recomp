/* DllPhotoViewer function @ CRO file offset 0x19b0, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Part of the 0x19a8-0x1a0c run of tiny leaf overrides (setter, return-true
 * stubs, bare bx lr) between the dtor-stage helpers and the first big function.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPhotoViewer 0x19b0 0x8 xy-decomp/armcc-matching/matched/DllPhotoViewer/0x19b0.c
 */

int stub_true(void *this)
{
    return 1;
}
