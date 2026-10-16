/*
 * DllBattle @ 0x4c7fc, size 0xc -- getter: returns the u16 field at this+0x160.
 * Match: 3/3 words (masked).
 * Style: natural -- plain field/address accessor, no codegen tricks
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c7fc 0xc xy-decomp/armcc-matching/matched/DllBattle/0x4c7fc.c
 */
unsigned short fn(char *self) { return *(unsigned short *)(self + 0x160); }
