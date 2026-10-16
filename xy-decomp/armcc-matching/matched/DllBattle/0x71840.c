/*
 * DllBattle @ 0x71840, size 0x10 -- zero-inits a 6-byte struct: u32 at +0, u16 at +4. Masked match 4/4.
 * Family "zero-init struct": see 0x76360.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x71840 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x71840.c
 */
void fn(unsigned char *p) { *(unsigned *)p = 0; *(unsigned short *)(p+4) = 0; }
