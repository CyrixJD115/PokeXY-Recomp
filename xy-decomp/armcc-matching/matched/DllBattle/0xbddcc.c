/*
 * DllBattle @ 0xbddcc, size 0x7c -- battle event-trigger predicate: if (g(3) == c && g(42) == 1 && g(78) == 0 && !h1((ushort)g(18)) && !h2(v, 1)) post(43, 2) (tail call); h1 = 0xa4814, h2 = 0x49c.
 * Masked match 31/31. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbddcc 0x7c xy-decomp/armcc-matching/matched/DllBattle/0xbddcc.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h1(int);
extern int h2(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(42) == 1 && g(78) == 0) { unsigned short v = g(18); if (h1(v) == 0 && h2(v, 1) == 0) fnA(43, 2); }
}
