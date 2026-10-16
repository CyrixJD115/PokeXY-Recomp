/* DllSequence function @ CRO file offset 0xef98, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Second of the three untried "return 1"-shaped leads flagged in this
 * module's README entry for 0x1135c. Right after a real field-lookup
 * function (ends 0xef94 with a tail branch to 0x9b0), a twin `return 1`
 * stub follows at 0xef98, then a run of bare `bx lr` single-word
 * weak-evidence stubs (0xef9c/0xefa0/0xefa4/0xefa8) before the next real
 * function at 0xefb4.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xef98 0x8 xy-decomp/armcc-matching/matched/DllSequence/0xef98.c
 */

int stub_true(void *this)
{
    return 1;
}
