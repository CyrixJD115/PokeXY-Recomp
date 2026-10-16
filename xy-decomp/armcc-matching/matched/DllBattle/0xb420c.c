/*
 * DllBattle @ 0xb420c, size 0x44 -- event predicate: if (g(3) == c && g(61) == 11) post(62, g(62) + 1) (tail call 0x658d4). Callees: 0x6b91c query, 0x658d4 post veneer.
 * Masked match 17/17.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb420c 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xb420c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(61) == 11) fnA(62, g(62) + 1);
}
