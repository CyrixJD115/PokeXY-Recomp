/*
 * DllBattle @ 0xc4244, size 0x3c -- battle event-trigger predicate: if (g(4) == c && g(22) == 9) post(50, 5120) (tail call, no return value).
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc4244 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xc4244.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && g(22) == 9) fnA(50, 5120);
}
