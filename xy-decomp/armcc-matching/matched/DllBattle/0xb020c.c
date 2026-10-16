/*
 * DllBattle @ 0xb020c, size 0x3c -- event predicate: if (g(3) == c && g(21) == 10) post(57, 8) (tail call 0x658d4, no return value). Callees: 0x6b91c query-by-id, 0x658d4 post-event veneer.
 * Masked match 15/15.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb020c 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xb020c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(21) == 10) fnA(57, 8);
}
