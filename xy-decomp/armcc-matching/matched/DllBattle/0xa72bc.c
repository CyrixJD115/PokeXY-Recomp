/*
 * DllBattle @ 0xa72bc, size 0x48 -- double-guarded report helper: if (g(3) == c && g(22) == 11) fnB(50, fnC(a)); (tail call, no return value).
 * Masked match 18/18. Constant-tweak sibling of the 0xa5de0 family (only the g(22) comparison constant differs); callees are the 0x6b91c getter, 0x4cbe4 and the 0x4ce84 veneer.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa72bc 0x48 xy-decomp/armcc-matching/matched/DllBattle/0xa72bc.c
 */
extern int g(int);
extern int fnC(int);
extern void fnB(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && g(22) == 11) fnB(50, fnC(a));
}
