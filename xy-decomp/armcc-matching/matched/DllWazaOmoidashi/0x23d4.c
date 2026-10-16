/*
 * DllWazaOmoidashi @ 0x23d4 (size 0x8) — 2/2 masked match.
 *
 * Trivial virtual-override stub, `return 1`, ignores `this`.
 *
 * Mid-module, right after a dispatcher's tail `b` at 0x23d0; immediately
 * followed by a bare `bx lr` single-word stub at 0x23dc (weak-evidence
 * class, untried) and a twin `return 1` stub at 0x23e0 (untried).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllWazaOmoidashi 0x23d4 0x8 \
 *     xy-decomp/armcc-matching/matched/DllWazaOmoidashi/0x23d4.c
 */
int fn(void *this)
{
    (void)this;
    return 1;
}
