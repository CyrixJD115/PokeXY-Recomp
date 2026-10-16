/*
 * DllBattle @ 0xc0764, size 0x4c -- battle event-trigger predicate: if (g(4) == c && g(27) == 0 && (unsigned)g(44) > 50) post(44, 50) (tail call).
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc0764 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xc0764.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && g(27) == 0 && (unsigned)g(44) > 50) fnA(44, 50);
}
