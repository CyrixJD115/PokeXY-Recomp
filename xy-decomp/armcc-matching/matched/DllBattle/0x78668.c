/*
 * DllBattle @ 0x78668, size 0x24 -- returns fn(0x100) > *p (unsigned compare of a helper result against a word read through p).
 * Masked match, all words.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x78668 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x78668.c
 */
extern unsigned fn484(unsigned);
int fn(void *o, unsigned *p) { return fn484(0x100) > *p; }
