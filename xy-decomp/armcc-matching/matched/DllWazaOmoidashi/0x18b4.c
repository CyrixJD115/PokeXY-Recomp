/*
 * DllWazaOmoidashi @ 0x18b4 (size 0x8) — 2/2 masked match.
 *
 * Trivial virtual-override stub, `return 1`, ignores `this`.
 *
 * Sits between two real call-dispatcher functions mid-module (previous
 * function ends with a tail `b` at 0x18b0, next real function starts at
 * 0x18bc).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllWazaOmoidashi 0x18b4 0x8 \
 *     xy-decomp/armcc-matching/matched/DllWazaOmoidashi/0x18b4.c
 */
int fn(void *this)
{
    (void)this;
    return 1;
}
