/*
 * Style: natural
 * DllGts @ 0x125cc, size 0x8, masked match 2/2.
 * Returns constant 0x1b.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x125cc 0x8 xy-decomp/armcc-matching/matched/DllGts/0x125cc.c
 */

int F(void *t){ return 0x1b; }
