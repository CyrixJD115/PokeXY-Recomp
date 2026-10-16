/*
 * DllBattle @ 0xbf148, size 0x44 -- battle event-trigger predicate: if (g(3) == c && h1((ushort)g(18)) != 0) post(50, 6144) (tail call); h1 = 0xa4714.
 * Masked match 17/17. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbf148 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xbf148.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h1(int);
void fn(int a, int b, int c) {
    if (g(3) == c && h1((unsigned short)g(18))) fnA(50, 6144);
}
