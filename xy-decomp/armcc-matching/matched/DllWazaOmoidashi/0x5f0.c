/*
 * DllWazaOmoidashi @ 0x5f0 (size 0x4) — 1/1 masked match.
 *
 * Trivial no-op void stub (single `bx lr`) — weak evidence, same class
 * already documented in DllDialogCommon/DllStrInput/etc: any void
 * function with no observable side effect compiles to this same one
 * word, so the match proves the body is at most this short, not that
 * it was empty.
 *
 * Sits right after the import-veneer table (0x180-0x5ec), immediately
 * before the module's first real function (a constructor-shaped
 * dispatcher at 0x5f4).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllWazaOmoidashi 0x5f0 0x4 \
 *     xy-decomp/armcc-matching/matched/DllWazaOmoidashi/0x5f0.c
 */
void fn(void *this)
{
    (void)this;
}
