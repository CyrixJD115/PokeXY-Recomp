/*
 * DllBattle @ 0xbe190, size 0x4c -- battle event-trigger predicate: if (g(a) == 16 && probe(88, 1) != 0) post(50, g(87) == 0 ? 5448 : 3072) (tail call).
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbe190 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xbe190.c
 */
extern int g(int);
extern void fnA(int, int);
extern int fnC(int, int);
void fn(int a) {
    if (g(a) == 16 && fnC(88, 1) != 0) fnA(50, g(87) == 0 ? 5448 : 3072);
}
