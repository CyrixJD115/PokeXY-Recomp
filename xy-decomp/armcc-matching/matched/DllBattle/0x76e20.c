/*
 * DllBattle @ 0x76e20, size 0x10 -- MATCHED (masked, push-less leaf).
 * bit-pack builder: ((a << 3) & 0x1f8) | 3 -- 6-bit field at bit 3 with a constant low tag 3. Identical bytes at 0x9e638 (also matched here).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x76e20 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x76e20.c
 */
unsigned fn(unsigned a) { return ((a << 3) & 0x1f8) | 3; }
