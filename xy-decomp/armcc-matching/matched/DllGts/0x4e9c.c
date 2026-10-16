/*
 * Style: natural
 * DllGts @ 0x4e9c size 0x10: set field 0x1c=8 ret 6
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x4e9c 0x10 xy-decomp/armcc-matching/matched/DllGts/0x4e9c.c
 */
int F(int *p){ p[7]=8; return 6; }
