/*
 * DllBattle @ 0x4c7d0, size 0x2c -- range-checked address getter: returns o + 0x467a (+17920+122) when i < 320, else calls assert stub fnA(0,0,0) and still returns the pointer.
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c7d0 0x2c xy-decomp/armcc-matching/matched/DllBattle/0x4c7d0.c
 */
extern void fnA(int a, int b, int c);
struct O { char pad[17920+122]; };
char *fn(char *o, unsigned i) {
    if (i >= 320) fnA(0,0,0);
    return o + 0x467a;
}
