/*
 * DllBattle @ 0x2420, size 0xc -- zeroes the word at this+8.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2420 0xc xy-decomp/armcc-matching/matched/DllBattle/0x2420.c
 */
void fn(int *o) { o[2] = 0; }
