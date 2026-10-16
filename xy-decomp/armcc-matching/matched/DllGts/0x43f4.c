/*
 * Style: natural
 * DllGts @ 0x43f4 size 0x10: clear field 0x5c ret 0x34
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x43f4 0x10 xy-decomp/armcc-matching/matched/DllGts/0x43f4.c
 */
int F(int *p){ p[23]=0; return 0x34; }
