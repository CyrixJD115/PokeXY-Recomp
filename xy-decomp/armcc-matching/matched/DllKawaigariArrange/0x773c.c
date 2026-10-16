/* DllKawaigariArrange @ 0x773c, size 0x8 — masked match 2/2.
 * Style: natural
 *
 * Trivial virtual-override stub: ignores `this`, returns 1 (boolean
 * true). Sits between two real functions: the previous function's own
 * literal-pool word at 0x7738, this 2-word leaf at 0x773c-0x7743, then
 * the next real function (push {r4,r5,r6,lr}) starting immediately at
 * 0x7744. Same recurring 1-2 word tail-of-vtable stub idiom already
 * proven in ~40 other Dll modules (see armcc-matching/README.md).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllKawaigariArrange 0x773c 0x8 \
 *       xy-decomp/armcc-matching/matched/DllKawaigariArrange/0x773c.c
 */

int fn(void *this) {
    (void)this;
    return 1;
}
