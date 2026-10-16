/*
 * DllBattle @ 0xc8754, size 0xc -- getter: returns the u16 field at this+0x150.
 * Match: 3/3 words (masked).
 * Style: natural -- plain field/address accessor, no codegen tricks
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc8754 0xc xy-decomp/armcc-matching/matched/DllBattle/0xc8754.c
 */
unsigned short fn(char *self) { return *(unsigned short *)(self + 0x150); }
