/* DllTradelist @ 0xc70, size 0x4 - trivial no-op void stub (single `bx lr`).
 * Match 1/1. Weak evidence: any void function with no observable side
 * effect compiles to the same one word, so the match alone doesn't prove
 * the real body was empty, only that it's at most this short.
 *
 * Sits right between the known-bad member-load-first dtor sibling at
 * 0xc3c (skipped) and the large 0xc74 dispatcher.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTradelist \
 *       0xc70 0x4 xy-decomp/armcc-matching/matched/DllTradelist/0xc70.c
 */
void fn(void *this) {
    (void)this;
}
