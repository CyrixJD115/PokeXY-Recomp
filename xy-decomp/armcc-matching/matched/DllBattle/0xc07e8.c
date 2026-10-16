/*
 * DllBattle @ 0xc07e8, size 0x40 -- battle event-trigger predicate: v = (uchar)g(4); if (v != c && h2(c, v)) post(54, 3072) (tail call); h2 is the 0x4c5a8 predicate.
 * Masked match 16/16. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc07e8 0x40 xy-decomp/armcc-matching/matched/DllBattle/0xc07e8.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h2(int, int);
void fn(int a, int b, int c) {
    unsigned char v = g(4); if (v != c && h2(c, v)) fnA(54, 3072);
}
