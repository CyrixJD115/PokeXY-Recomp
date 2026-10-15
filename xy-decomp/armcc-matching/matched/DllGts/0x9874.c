/*
 * DllGts @ 0x9874, size 0xc, masked match 3/3.
 * Setter: this->byte@0x3c = 1.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x9874 0xc xy-decomp/armcc-matching/matched/DllGts/0x9874.c
 */

typedef struct O { unsigned char p[0x3c]; unsigned char f3c; } O;
void F(O *t){ t->f3c = 1; }
