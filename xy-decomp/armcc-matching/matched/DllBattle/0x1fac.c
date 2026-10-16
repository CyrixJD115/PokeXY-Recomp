/*
 * DllBattle @ 0x1fac, size 0x10 -- masked match 4/4.
 *
 * Trivial boolean predicate: `return x == 0;` on the second argument,
 * ignoring `this` (first arg). armcc 5.04u5 compiles the direct
 * equality return straight to cmp/moveq/movne with no extra scheduling
 * trap here (unlike some sibling predicates elsewhere in this project
 * where source phrasing of the comparison changes instruction order).
 *
 * Immediately follows the zero-init helper at 0x1f8c (same module).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle \
 *     0x1fac 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x1fac.c
 */

int fn(void *o, int x) {
    return x == 0;
}
