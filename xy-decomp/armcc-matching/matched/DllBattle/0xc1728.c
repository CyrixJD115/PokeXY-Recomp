/*
 * DllBattle @ 0xc1728, size 0x4c -- battle event-trigger predicate: if (g(3) == c && h1((ushort)g(18)) == 0) post(25, g(25) + 1) (tail call).
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc1728 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xc1728.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h1(int);
void fn(int a, int b, int c) {
    if (g(3) == c && h1((unsigned short)g(18)) == 0) fnA(25, g(25) + 1);
}
