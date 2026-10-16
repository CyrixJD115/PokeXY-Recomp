/*
 * DllBattle @ 0xc9e58, size 0x10 -- zero-inits a 5-byte struct: u32 at +0, u8 at +4. Masked match 4/4.
 * Family "zero-init struct": see 0x76360.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9e58 0x10 xy-decomp/armcc-matching/matched/DllBattle/0xc9e58.c
 */
void fn(unsigned char *p) { *(unsigned *)p = 0; p[4] = 0; }
