/*
 * DllWazaOmoidashi @ 0x3514 (size 0x8) — 2/2 masked match.
 *
 * Trivial virtual-override stub, `return 1`, ignores `this`.
 *
 * Right after a dispatcher/constructor's own literal-pool word at
 * 0x3510; followed by four bare `bx lr` single-word stubs
 * (0x351c/0x3520/0x3524/0x3528, weak-evidence class, unclaimed) before
 * the next real function at 0x352c.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllWazaOmoidashi 0x3514 0x8 \
 *     xy-decomp/armcc-matching/matched/DllWazaOmoidashi/0x3514.c
 */
int fn(void *this)
{
    (void)this;
    return 1;
}
