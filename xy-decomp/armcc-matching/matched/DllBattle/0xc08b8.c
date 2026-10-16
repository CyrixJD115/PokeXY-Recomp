/*
 * DllBattle @ 0xc08b8, size 0x3c -- battle event-trigger predicate: if (g(4) == c && g(3) != c) post(66, 1) (tail call).
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc08b8 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xc08b8.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && g(3) != c) fnA(66, 1);
}
