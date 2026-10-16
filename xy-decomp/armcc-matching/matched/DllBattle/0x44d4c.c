/*
 * DllBattle @ 0x44d4c, size 0x18 -- calls a helper and returns its logical negation (1 if it returned 0).
 * Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x44d4c 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x44d4c.c
 */
extern int g(void);
int fn(void) { if (g()) return 0; return 1; }
