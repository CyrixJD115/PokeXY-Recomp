/*
 * DllBattle @ 0xbf300, size 0x44 -- battle event-trigger predicate: if (g(3) == c && h((unsigned char)g(57)) == 3) post(54, 8192) (tail call, no return value); h is an 0x9e58c lookup helper.
 * Masked match 17/17. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbf300 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xbf300.c
 */
extern int g(int);
extern int h(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && h((unsigned char)g(57)) == 3) fnA(54, 8192);
}
