/*
 * DllBattle @ 0xc87a0, size 0x14 -- bounds-checked byte array getter: returns p[i] when
 * i < this->count (u8 at +25), else the default 6. Masked match 5/5 (ldrbhi/movls #6).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc87a0 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc87a0.c
 */
unsigned fn(unsigned char *p, unsigned i) {
    if (i < p[25]) return p[i];
    return 6;
}
