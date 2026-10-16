/*
 * DllBattle @ 0xc9e30, size 0x14 -- zero-inits a 10-byte struct: u32 at +0, u32 at +4, u16 at +8. Masked match 5/5.
 * Family "zero-init struct": see 0x76360.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9e30 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc9e30.c
 */
void fn(unsigned char *p) {
    *(unsigned *)p = 0; *(unsigned *)(p+4) = 0; *(unsigned short *)(p+8) = 0;
}
