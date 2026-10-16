/* DllBox function @ CRO file offset 0xa00, size 0x4 — MATCHED (masked, 1/1).
 * Trivial no-op void stub (single `bx lr`) — weak evidence, see note below.
 * Style: natural
 *
 * Sits between the `return 1` stub cluster (0x9e8/0x9f0/0x9f8) and another
 * `return 1` stub at 0xa04, in the run of bare `bx lr` single-word stubs
 * (0xa00/0xa08/0xa0c/0xa10) at the tail of the import-veneer table.
 * Weak-evidence match: any void function with no observable side effect
 * compiles to this same one word, so the match alone doesn't prove the
 * real body was empty — only that it's at most this short.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0xa00 0x4 xy-decomp/armcc-matching/matched/DllBox/0xa00.c
 */

void noop(void *this)
{
}
