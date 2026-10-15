/*
 * DllBattle @ 0x567e0, size 0x18 -- byte-splat fill of an 8-byte field: replicate the low byte of
 * `v` into all 4 bytes of a word and store it twice (dst[0..3], dst[4..7]). Masked match 6/6.
 * Family "byte-splat fill" (push-less leaf): 0x567e0 (8 bytes), 0x6fd18 (u32 + u16 = 6 bytes),
 * 0xc9e84 (u32 + u32 + u16 = 10 bytes).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x567e0 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x567e0.c
 */
void fn(unsigned char *p, unsigned v) {
    unsigned w = v & 0xff;
    w |= w << 8;
    w |= w << 16;
    *(unsigned *)p = w;
    *(unsigned *)(p + 4) = w;
}
