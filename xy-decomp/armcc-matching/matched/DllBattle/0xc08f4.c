/*
 * DllBattle @ 0xc08f4, size 0x4c -- battle event-trigger predicate: if (g(4) == c && g(3) != c && g(30) != 8) post(66, 1) (tail call).
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc08f4 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xc08f4.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && g(3) != c && g(30) != 8) fnA(66, 1);
}
