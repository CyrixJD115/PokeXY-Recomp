/*
 * DllBattle @ 0x4c278, size 0xc -- getter: returns the u16 field at this+0x162.
 * Match: 3/3 words (masked).
 * Style: natural -- plain field/address accessor, no codegen tricks
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c278 0xc xy-decomp/armcc-matching/matched/DllBattle/0x4c278.c
 */
unsigned short fn(char *self) { return *(unsigned short *)(self + 0x162); }
