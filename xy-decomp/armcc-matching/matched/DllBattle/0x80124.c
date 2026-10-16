/*
 * DllBattle @ 0x80124, size 0x1c -- returns whether fn(u16 at +198, 3) is non-zero (bool wrapper over a helper call).
 * Masked match, all words.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x80124 0x1c xy-decomp/armcc-matching/matched/DllBattle/0x80124.c
 */
extern int fn494(unsigned, int);
int fn(unsigned char *o) { return fn494(*(unsigned short *)(o + 198), 3) != 0; }
