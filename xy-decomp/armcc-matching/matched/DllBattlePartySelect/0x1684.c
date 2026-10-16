/* DllBattlePartySelect@0x1684, size 0x10 -- 4/4 masked match.
 * Style: natural
 *
 * Trivial prologue-less leaf predicate: ignores both `this` (r0) and the
 * second argument (r1), tests only the third argument against zero
 * (`return c == 0`). Phrasing note: `return c == 0;` (or a ternary) emits
 * the two conditional movs in the wrong order (moveq before movne); the
 * plain `if (c != 0) return 0; return 1;` predicate form reproduces the
 * retail order (movne before moveq). Audit 2026-10-16: the earlier
 * `int r; if (c != 0) r = 0; else r = 1; return r;` local was unnecessary.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattlePartySelect 0x1684 0x10 \
 *       xy-decomp/armcc-matching/matched/DllBattlePartySelect/0x1684.c
 */
int fn(void *a, int b, int c) {
    if (c != 0) return 0;
    return 1;
}
