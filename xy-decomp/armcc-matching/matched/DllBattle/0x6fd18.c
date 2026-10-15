/*
 * DllBattle @ 0x6fd18, size 0x18 -- byte-splat fill of a 6-byte field: replicate the low byte of
 * `v` into a word, store it as u32 at +0 and as u16 at +4. Masked match 6/6.
 * Family "byte-splat fill": see 0x567e0.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x6fd18 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x6fd18.c
 */
void fn(unsigned char *p, unsigned v) {
    unsigned w = v & 0xff;
    w |= w << 8;
    w |= w << 16;
    *(unsigned *)p = w;
    *(unsigned short *)(p + 4) = w;
}
