/*
 * DllBattle @ 0xb7b14, size 0x3c -- event predicate: if (g(3) == c && g(34) == 5) post(67, 0) (tail call 0x658d4, no return value). Callees: 0x6b91c query-by-id, 0x658d4 post-event veneer.
 * Masked match 15/15.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb7b14 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xb7b14.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(34) == 5) fnA(67, 0);
}
