/*
 * Style: natural
 * DllGts @ 0x43dc size 0xc: setter byte@0xe0=1
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x43dc 0xc xy-decomp/armcc-matching/matched/DllGts/0x43dc.c
 */
void F(unsigned char *p){ p[0xe0]=1; }
