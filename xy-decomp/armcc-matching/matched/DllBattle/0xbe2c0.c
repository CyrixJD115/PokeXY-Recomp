/*
 * DllBattle @ 0xbe2c0, size 0x3c -- battle event-trigger predicate: if (h2(c, (uchar)g(3)) != 0) post(54, 4506) (tail call); h2 is the 0x4c5a8 predicate.
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbe2c0 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xbe2c0.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h2(int, int);
void fn(int a, int b, int c) {
    if (h2(c, (unsigned char)g(3))) fnA(54, 4506);
}
