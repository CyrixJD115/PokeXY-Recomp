/*
 * DllSparringMenu @ 0x5b14, size 0x8 — masked match 2/2.
 *
 * Trivial virtual-override stub, `return 1`, ignores `this`. Sits right
 * after the SetIsCallDraw setter at 0x5b0c (already matched), between
 * two small conditional field-set leaves at 0x5b1c and 0x5b58 (both
 * near-identical: clamp a value 1/16/17 -> 2/3/4 and store into
 * this-0x84+0x88 or this-0x84+0x8c, untried).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSparringMenu 0x5b14 0x8 \
 *       xy-decomp/armcc-matching/matched/DllSparringMenu/0x5b14.c
 */
int fn(void *this) {
    return 1;
}
