/*
 * DllBattle @ 0xc6d24, size 0x14 -- ctor stage: call base ctor, then this->[0x48] = this (self pointer).
 * Masked match.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc6d24 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc6d24.c
 */
extern void *base(void *);
void fn(void **p) { base(p); p[18] = p; }
