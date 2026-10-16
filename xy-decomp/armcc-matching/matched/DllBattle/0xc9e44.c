/*
 * DllBattle @ 0xc9e44, size 0x14 -- zero-inits a 12-byte struct: three zero u32 stores (+0,+4,+8). Masked match 5/5.
 * Family "zero-init struct": see 0x76360.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9e44 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc9e44.c
 */
void fn(unsigned *p) { p[0] = 0; p[1] = 0; p[2] = 0; }
