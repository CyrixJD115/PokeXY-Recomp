/*
 * DllBattle @ 0xbd42c, size 0x34 -- battle event-trigger predicate: if (g(2) == c) post(33, g(33) << 1) (tail call): doubles query 33.
 * Masked match 13/13. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbd42c 0x34 xy-decomp/armcc-matching/matched/DllBattle/0xbd42c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(2) == c) fnA(33, g(33) << 1);
}
