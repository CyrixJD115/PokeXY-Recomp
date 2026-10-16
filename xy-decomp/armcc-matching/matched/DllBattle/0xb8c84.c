/*
 * DllBattle @ 0xb8c84, size 0x40 -- battle event-trigger predicate: if (g(2) == c && (uchar)g(35) == 2) post(66, 1) (tail call).
 * Masked match 16/16. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb8c84 0x40 xy-decomp/armcc-matching/matched/DllBattle/0xb8c84.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(2) == c && (unsigned char)g(35) == 2) fnA(66, 1);
}
