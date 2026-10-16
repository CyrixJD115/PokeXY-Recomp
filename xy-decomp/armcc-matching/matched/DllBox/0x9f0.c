/* DllBox function @ CRO file offset 0x9f0, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * Second of the three identical `return 1` stubs in the tail-of-import-
 * veneer-table cluster (0x9e8/0x9f0/0x9f8) — same bytes as the already-
 * matched 0x9e8, confirmed identical in the disassembly before compiling.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x9f0 0x8 xy-decomp/armcc-matching/matched/DllBox/0x9f0.c
 */

int stub_true(void *this)
{
    return 1;
}
