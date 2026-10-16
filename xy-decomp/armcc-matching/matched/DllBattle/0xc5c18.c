/*
 * DllBattle @ 0xc5c18, size 0x28 -- clears a 26-byte struct: zero byte at +24 and +25, then six zero u32 stores (+0..+20). Masked match 10/10.
 * Family "zero-init struct": see 0x76360.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc5c18 0x28 xy-decomp/armcc-matching/matched/DllBattle/0xc5c18.c
 */
void fn(unsigned char *p) {
    p[24] = 0; p[25] = 0;
    *(unsigned *)p = 0; *(unsigned *)(p+4) = 0; *(unsigned *)(p+8) = 0;
    *(unsigned *)(p+12) = 0; *(unsigned *)(p+16) = 0; *(unsigned *)(p+20) = 0;
}
