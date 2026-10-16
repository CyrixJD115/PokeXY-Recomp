/*
 * DllBattle @ 0xb09d0, size 0x40 -- event predicate: if (g(3) == c) { v = g(31); adjust(&v, 7); post(31, v) } (tail call 0x658d4). Callees: 0x6b91c query, 0x4b990 in-place adjust helper, 0x658d4 post veneer.
 * Masked match 16/16.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb09d0 0x40 xy-decomp/armcc-matching/matched/DllBattle/0xb09d0.c
 */
extern int g(int);
extern void fnA(int, int);
extern void fnH(int *, int);
void fn(int a, int b, int c) {
    if (g(3) == c) {
        int v = g(31);
        fnH(&v, 7);
        fnA(31, v);
    }
}
