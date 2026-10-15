/*
 * DllBattle @ 0xc9e84, size 0x1c -- byte-splat fill of a 10-byte field: replicate the low byte of
 * `v` into a word, store it as u32 at +0, u32 at +4, u16 at +8. Masked match 7/7.
 * Family "byte-splat fill": see 0x567e0.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9e84 0x1c xy-decomp/armcc-matching/matched/DllBattle/0xc9e84.c
 */
void fn(unsigned char *p, unsigned v) {
    unsigned w = v & 0xff;
    w |= w << 8;
    w |= w << 16;
    *(unsigned *)p = w;
    *(unsigned *)(p + 4) = w;
    *(unsigned short *)(p + 8) = w;
}
