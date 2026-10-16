/*
 * DllBattle @ 0xc297c, size 0x44 -- battle event-trigger predicate: if (g(3) == c && g(22) == 2) post(25, g(25) + 1) (tail call).
 * Masked match 17/17. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc297c 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xc297c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(22) == 2) fnA(25, g(25) + 1);
}
