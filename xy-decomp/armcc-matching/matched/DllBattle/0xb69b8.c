/*
 * DllBattle @ 0xb69b8, size 0x3c -- event predicate: if (g(4) == c && g(3) == c) post(32, 10) (tail call 0x658d4). Callees: 0x6b91c query, 0x658d4 post veneer.
 * Masked match 15/15.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb69b8 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xb69b8.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && g(3) == c) fnA(32, 10);
}
