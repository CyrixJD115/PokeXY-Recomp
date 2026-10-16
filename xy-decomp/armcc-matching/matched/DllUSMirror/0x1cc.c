/* DllUSMirror function @ CRO file offset 0x1cc, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Part of a run of three identical return-0 stubs (0x1cc/0x1d4/0x1dc)
 * sitting in the tail of the import-veneer table's leaf-stub cluster
 * (0x1c8-0x1fc), right before a real dispatcher function at 0x200 --
 * same shape already documented across every other Dll module mined
 * so far (default virtual-method overrides on a shared base class).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSMirror 0x1cc 0x8 xy-decomp/armcc-matching/matched/DllUSMirror/0x1cc.c
 */

int stub_false(void *this)
{
    return 0;
}
