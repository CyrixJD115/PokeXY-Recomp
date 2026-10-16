/*
 * DllBattle @ 0xb8dcc, size 0x2c -- guarded tail call: if (g(2) == c) fnY(a); (no return value).
 * Masked match 11/11. Comparison wrapper around the 0x6b91c getter; callee is a veneer (b masked).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb8dcc 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xb8dcc.c
 */
extern int g(int);
extern void fnY(int);
void fn(int a, int b, int c) {
    if (g(2) == c) fnY(a);
}
