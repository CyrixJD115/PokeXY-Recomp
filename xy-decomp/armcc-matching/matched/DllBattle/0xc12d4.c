/*
 * DllBattle @ 0xc12d4, size 0x4c -- battle event-trigger predicate: if (g(3) == c && h2((ushort)g(18), 7) != 0) post(50, 4915) (tail call); h2 = 0x49c.
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc12d4 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xc12d4.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h2(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && h2((unsigned short)g(18), 7)) fnA(50, 4915);
}
