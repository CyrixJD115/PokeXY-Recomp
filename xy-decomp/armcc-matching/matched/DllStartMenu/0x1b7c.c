/* DllStartMenu @ 0x1b7c, size 0x4 — masked match 1/1.
 * Style: natural
 *
 * One-line: trivial no-op void stub (single `bx lr`) — weak evidence, any
 * side-effect-free void function compiles to the same one word; this only
 * proves the real body is at most this short, not that it was empty. One
 * of a run of five identical 1-word stubs at 0x1b7c-0x1b8c (virtual
 * no-op overrides); only this offset is claimed here.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStartMenu \
 *       0x1b7c 0x4 xy-decomp/armcc-matching/matched/DllStartMenu/0x1b7c.c
 */
void func_1b7c(void) { }
