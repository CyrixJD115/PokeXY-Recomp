/* DllVSDemo function @ CRO file offset 0x1a60, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after a destructor-stage helper ending at 0x1a54 (vtbl
 * install + tail-call to a local function) and its own one-word literal
 * pool at 0x1a58, plus a bare `bx lr` weak-evidence stub at 0x1a5c.
 * Same recurring tail-of-vtable leaf-stub shape documented in every
 * other mined module.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllVSDemo 0x1a60 0x8 xy-decomp/armcc-matching/matched/DllVSDemo/0x1a60.c
 */

int stub_true(void *this)
{
    return 1;
}
