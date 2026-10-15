/*
 * DllWazaOboe @ 0x804, size 0x4, masked match 1/1.
 * Trivial no-op void stub (single `bx lr`) — weak evidence: any void
 * function with no observable side effect compiles to this same one word,
 * so the match only proves the real body is at most this short, not that
 * it's empty. One of the alternating `return 1`/bare-`bx lr` leaf stubs in
 * the 0x7ec-0x837 cluster right after the import-veneer table.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllWazaOboe \
 *       0x804 0x4 xy-decomp/armcc-matching/matched/DllWazaOboe/0x804.c
 */
void fn(void *this) {
    (void)this;
}
