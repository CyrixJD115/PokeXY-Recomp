/*
 * DllBattle @ 0xb7850, size 0x2c -- guarded report helper: if (getter(3) == c) reportA(50, 6144) (tail call, no return value).
 * Masked match 11/11. Comparison wrapper around the 0x6b91c getter; the callee is the 0x4ce84 veneer.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb7850 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xb7850.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnA(50, 6144);
}
