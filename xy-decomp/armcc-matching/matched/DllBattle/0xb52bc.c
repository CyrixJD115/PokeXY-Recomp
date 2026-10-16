/*
 * DllBattle @ 0xb52bc, size 0x48 -- event predicate: if (g(3) == c) { v = g(80) (unsigned); post(49, v < 40 ? 40 : v < 70 ? 80 : 120) } (tail call 0x658d4). Callees: 0x6b91c query, 0x658d4 post veneer.
 * Masked match 18/18.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb52bc 0x48 xy-decomp/armcc-matching/matched/DllBattle/0xb52bc.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) {
        unsigned v = g(80);
        fnA(49, v < 40 ? 40 : v < 70 ? 80 : 120);
    }
}
