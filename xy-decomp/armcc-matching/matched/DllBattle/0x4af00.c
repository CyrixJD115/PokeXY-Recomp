/*
 * DllBattle @ 0x4af00, size 0x30 -- range-checked predicate: if x >= 2 calls the error/assert helper (0,0,0),
 * then returns (x == 0).
 * Masked match 12/12.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4af00 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x4af00.c
 */
extern void fnA(int a, int b, int c);
int fn(unsigned int x) {
    if (x >= 2) fnA(0, 0, 0);
    return x == 0;
}
