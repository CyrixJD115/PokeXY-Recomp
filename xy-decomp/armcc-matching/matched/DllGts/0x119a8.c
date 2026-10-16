/*
 * Style: natural
 * DllGts @ 0x119a8, size 0xc, masked match 3/3.
 * Setter: this->byte@0x40 = 1.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x119a8 0xc xy-decomp/armcc-matching/matched/DllGts/0x119a8.c
 */

typedef struct O { unsigned char p[0x40]; unsigned char f40; } O;
void F(O *t){ t->f40 = 1; }
