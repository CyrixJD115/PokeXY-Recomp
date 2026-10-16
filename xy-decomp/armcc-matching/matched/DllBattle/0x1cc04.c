/*
 * DllBattle @ 0x1cc04, size 0xc -- returns the address of the embedded sub-object at this+0xca4 (add 0xc00; add 0xa4).
 * Match: 3/3 words (masked).
 * Style: natural -- plain field/address accessor, no codegen tricks
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x1cc04 0xc xy-decomp/armcc-matching/matched/DllBattle/0x1cc04.c
 */
char *fn(char *self) { return self + 0xca4; }
