/*
 * DllBattle @ 0xab408, size 0x2c -- guarded report helper: if (getter(2) == c) reportB(66, 1) (tail call, no return value).
 * Masked match 11/11. Comparison wrapper around the 0x6b91c getter; the callee is the 0x658d4 veneer.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xab408 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xab408.c
 */
extern int g(int);
extern void fnB(int, int);
void fn(int a, int b, int c) {
    if (g(2) == c) fnB(66, 1);
}
