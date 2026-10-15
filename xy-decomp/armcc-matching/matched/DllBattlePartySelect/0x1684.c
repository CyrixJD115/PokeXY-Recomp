/* DllBattlePartySelect@0x1684, size 0x10 -- 4/4 masked match.
 *
 * Trivial prologue-less leaf predicate: ignores both `this` (r0) and the
 * second argument (r1), tests only the third argument against zero
 * (`return c == 0`). Non-obvious bit: writing it as `return c == 0;` or
 * as a ternary both compile to the right two conditional movs but in
 * the WRONG order (moveq before movne); the retail binary emits movne
 * before moveq. An explicit `if (c != 0) r = 0; else r = 1;` reproduces
 * the exact order armcc 5.04u5 chose here -- same residue-class lesson
 * as the project's "don't assume the obvious source phrasing" notes.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattlePartySelect 0x1684 0x10 \
 *       xy-decomp/armcc-matching/matched/DllBattlePartySelect/0x1684.c
 */
int fn(void *a, int b, int c) {
    int r;
    if (c != 0) r = 0; else r = 1;
    return r;
}
