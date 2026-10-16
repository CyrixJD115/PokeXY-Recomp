/* DllSequence function @ CRO file offset 0xee90, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * One of the three untried "return 1"-shaped leads flagged in this
 * module's README entry for 0x1135c. Sits mid-module, inside a run of
 * bare `bx lr` single-word weak-evidence stubs on both sides
 * (0xee88/0xee8c before it, 0xee94/0xee98/0xee9c/0xeea0/0xeea8/0xeeac
 * after) — same recurring tail-of-vtable trivial-stub-cluster shape
 * documented for every other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xee90 0x8 xy-decomp/armcc-matching/matched/DllSequence/0xee90.c
 */

int stub_true(void *this)
{
    return 1;
}
