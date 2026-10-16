/*
 * DllBattle @ 0xaf710, size 0x34 -- event predicate: if (g(3) == c) post(60, g(4) & 255) (tail call 0x658d4). Callees: 0x6b91c query, 0x658d4 post veneer.
 * Masked match 13/13.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xaf710 0x34 xy-decomp/armcc-matching/matched/DllBattle/0xaf710.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnA(60, g(4) & 255);
}
