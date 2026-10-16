/*
 * DllBattle @ 0xbdd94, size 0x38 -- battle event-trigger predicate: if (h2((uchar)g(2), c) != 0) post(83, 1) (tail call); h2 is the 0x4c5a8 predicate.
 * Masked match 14/14. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbdd94 0x38 xy-decomp/armcc-matching/matched/DllBattle/0xbdd94.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h2(int, int);
void fn(int a, int b, int c) {
    if (h2((unsigned char)g(2), c)) fnA(83, 1);
}
