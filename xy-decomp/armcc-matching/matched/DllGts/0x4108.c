/*
 * Style: natural
 * DllGts @ 0x4108 size 0xc: setter byte@0x44=1
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x4108 0xc xy-decomp/armcc-matching/matched/DllGts/0x4108.c
 */
void F(unsigned char *p){ p[0x44]=1; }
