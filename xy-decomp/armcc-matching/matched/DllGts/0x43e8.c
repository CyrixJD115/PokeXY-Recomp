/*
 * Style: natural
 * DllGts @ 0x43e8, size 0xc, masked match 3/3.
 * Setter: this->byte@0xe0 = 1.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x43e8 0xc xy-decomp/armcc-matching/matched/DllGts/0x43e8.c
 */

typedef struct O { unsigned char p[0xe0]; unsigned char fe0; } O;
void F(O *t){ t->fe0 = 1; }
