/*
 * DllBattle @ 0xb9b44, size 0x2c -- guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value).
 * Masked match 11/11. Comparison wrapper around the 0x6b91c getter; the callee is the 0x658d4 veneer.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb9b44 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xb9b44.c
 */
extern int g(int);
extern void fnB(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnB(83, 1);
}
