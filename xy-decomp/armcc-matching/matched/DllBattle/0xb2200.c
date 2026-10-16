/*
 * DllBattle @ 0xb2200, size 0x48 -- event predicate: if (d[0] == 2 && g(3) == c && g(22) == 12) post(50, 0x2000) (tail call 0x4ce84; d = context int*). Callees: 0x6b91c query, 0x4ce84 post veneer.
 * Masked match 18/18.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb2200 0x48 xy-decomp/armcc-matching/matched/DllBattle/0xb2200.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c, int *d) {
    if (d[0] == 2 && g(3) == c && g(22) == 12) fnA(50, 8192);
}
