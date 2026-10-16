/*
 * DllBattle @ 0x5f848, size 0x18 -- calls a helper and returns true if it returned 0.
 * Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x5f848 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x5f848.c
 */
extern int g(void);
int fn(void) { return g() == 0; }
