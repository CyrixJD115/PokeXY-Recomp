/*
 * DllBattle @ 0x742b0, size 0x10 -- calls a helper (result discarded) and returns 0.
 * Masked match 4/4.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x742b0 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x742b0.c
 */
extern void g(void);
int fn(void) { g(); return 0; }
