/*
 * DllBattle @ 0xbf010, size 0x44 -- battle event-trigger predicate: if (g(4) == c && ((uchar)g(22) == 14 || == 9)) post(54, 2048) (tail call).
 * Masked match 17/17. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbf010 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xbf010.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c) {
        unsigned char v = g(22);
        if (v == 14 || v == 9) fnA(54, 2048);
    }
}
