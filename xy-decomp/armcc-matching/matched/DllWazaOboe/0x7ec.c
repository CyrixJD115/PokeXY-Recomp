/*
 * DllWazaOboe @ 0x7ec, size 0x8, masked match 2/2.
 * Trivial virtual-override stub: ignores `this`, returns 1.
 * One of a run of near-identical 1-2 word leaf stubs (0x7ec-0x837) right
 * after the import-veneer table (0x180-0x7eb) — same recurring shape
 * documented across every other Dll module mined so far.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllWazaOboe \
 *       0x7ec 0x8 xy-decomp/armcc-matching/matched/DllWazaOboe/0x7ec.c
 */
int fn(void *this) {
    (void)this;
    return 1;
}
