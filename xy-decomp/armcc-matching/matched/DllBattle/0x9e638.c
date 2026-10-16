/*
 * DllBattle @ 0x9e638, size 0x10 -- MATCHED (masked, push-less leaf).
 * bit-pack builder: ((a << 3) & 0x1f8) | 3 -- byte-identical sibling of 0x76e20.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x9e638 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x9e638.c
 */
unsigned fn(unsigned a) { return ((a << 3) & 0x1f8) | 3; }
