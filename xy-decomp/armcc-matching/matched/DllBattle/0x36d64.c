/*
 * DllBattle @ 0x36d64, size 0x10 -- chained call: fnB(fnA(o)), second call is a tail call
 * (void result; the first call's return value is forwarded as the argument).
 * Masked match 4/4.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x36d64 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x36d64.c
 */
extern void *fnA(void *o);
extern void fnB(void *o);
void fn(void *o) {
    fnB(fnA(o));
}
