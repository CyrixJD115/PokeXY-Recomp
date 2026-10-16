/* DllVSDemo function @ CRO file offset 0x408, size 0x4 — MATCHED (masked, 1/1).
 * Trivial virtual-override stub: single `bx lr`, a true no-op void override.
 *
 * One of a run of three bare `bx lr` single-word stubs (0x408/0x40c/0x410)
 * right after the five `return 1` stubs at 0x3e0-0x400 (0x3e0 already
 * matched). Weakest-evidence match in the set: any no-op void body with
 * no observable side effect compiles to the same single word.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllVSDemo 0x408 0x4 xy-decomp/armcc-matching/matched/DllVSDemo/0x408.c
 */

void stub_noop(void *this)
{
}
