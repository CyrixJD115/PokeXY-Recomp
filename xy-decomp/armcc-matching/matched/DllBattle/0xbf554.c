/*
 * DllBattle @ 0xbf554, size 0x3c -- battle event-trigger predicate: if (g(3) == c && g(21) == 7) post(76, 1) (tail call, no return value).
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbf554 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xbf554.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(21) == 7) fnA(76, 1);
}
