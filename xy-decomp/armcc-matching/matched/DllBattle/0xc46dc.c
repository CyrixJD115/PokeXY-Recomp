/*
 * DllBattle @ 0xc46dc, size 0x2c -- guarded report helper: if (g(3) == c) fnA(70, 1) (tail call, no return value).
 * Masked match 11/11. Comparison wrapper around the 0x6b91c getter; constant-tweak sibling of the 0xa5db4 family; callee is a veneer.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc46dc 0x2c xy-decomp/armcc-matching/matched/DllBattle/0xc46dc.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnA(70, 1);
}
