/*
 * DllBattle @ 0x4d2f4, size 0x24 -- call fnB(fnA(o, 14), p); returns the result, or 1 if it was 0.
 * Masked match 9/9.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4d2f4 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x4d2f4.c
 */
extern void *fnA(void *o, int k);
extern int fnB(void *o, void *p);
int fn(void *o, void *p) {
    int r = fnB(fnA(o, 14), p);
    if (r == 0) r = 1;
    return r;
}
