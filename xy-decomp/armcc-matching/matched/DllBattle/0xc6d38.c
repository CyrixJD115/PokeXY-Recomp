/*
 * DllBattle @ 0xc6d38, size 0x14 -- ctor stage: call a different base ctor, then this->[0x48] = this.
 * Masked match.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc6d38 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc6d38.c
 */
extern void *base(void *);
void fn(void **p) { base(p); p[18] = p; }
