/*
 * DllBattle @ 0xc368c, size 0x3c -- battle event-trigger predicate: if (g(2) == c && g(51) > 0) post(66, 1) (tail call).
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc368c 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xc368c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(2) == c && g(51) > 0) fnA(66, 1);
}
