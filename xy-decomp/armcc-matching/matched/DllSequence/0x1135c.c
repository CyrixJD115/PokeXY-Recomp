/* DllSequence function @ CRO file offset 0x1135c, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits inside a run of near-identical 1-2 word leaf stubs at 0x1134c-
 * 0x1136f (bare `bx lr` weak-evidence stubs at 0x1134c/0x11350/0x11354/
 * 0x11358, this `return 1` stub, more bare `bx lr` stubs at 0x11360/
 * 0x11364/0x11368/0x1136c), right after the previous function's own
 * one-word literal pool at 0x11344-0x11348 and its `pop {r4, pc}`
 * epilogue at 0x11340. Same recurring tail-of-vtable leaf-stub cluster
 * shape documented for every other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0x1135c 0x8 xy-decomp/armcc-matching/matched/DllSequence/0x1135c.c
 */

int stub_true(void *this)
{
    return 1;
}
