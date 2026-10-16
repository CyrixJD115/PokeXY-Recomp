/*
 * DllBattle @ 0x79624, size 0x24 -- returns fn(0x100) != *p (helper result compared for inequality with a word read through p).
 * Masked match, all words.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x79624 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x79624.c
 */
extern unsigned fn484(unsigned);
int fn(void *o, unsigned *p) { return fn484(0x100) != *p; }
