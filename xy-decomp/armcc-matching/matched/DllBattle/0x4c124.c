/*
 * DllBattle @ 0x4c124, size 0x1c -- tail-call wrapper: returns fnB(fnA(o, 14), p).
 * (fnA fetches sub-object/slot 14 of o; the result and the 2nd arg are forwarded to fnB.)
 * Masked match 7/7.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c124 0x1c xy-decomp/armcc-matching/matched/DllBattle/0x4c124.c
 */
extern void *fnA(void *o, int k);
extern int fnB(void *o, void *p);
int fn(void *o, void *p) {
    return fnB(fnA(o, 14), p);
}
