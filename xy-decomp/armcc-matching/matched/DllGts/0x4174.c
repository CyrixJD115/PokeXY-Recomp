/*
 * Style: natural
 * DllGts @ 0x4174, size 0xc, masked match 3/3.
 * Setter: this->byte@0x44 = 1.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x4174 0xc xy-decomp/armcc-matching/matched/DllGts/0x4174.c
 */

typedef struct O { unsigned char p[0x44]; unsigned char f44; } O;
void F(O *t){ t->f44 = 1; }
