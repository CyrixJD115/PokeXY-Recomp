/*
 * DllBattle @ 0x4c948, size 0x24 -- sub-object lookup: p = fnA(this + 0x437c); tail-calls fnB(this, p) and returns its result.
 * Masked match 9/9.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c948 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x4c948.c
 */
extern void *fnA(void *p);
extern void *fnB(void *o, void *p);
void *fn(char *o) {
    return fnB(o, fnA(o + 0x437c));
}
