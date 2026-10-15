/*
 * DllGts @ 0x5320, size 0x10, masked match 4/4.
 * Stores 0x1d into this->word@0x1c, returns 6.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x5320 0x10 xy-decomp/armcc-matching/matched/DllGts/0x5320.c
 */

typedef struct O { unsigned char p[0x1c]; int f1c; } O;
int F(O *t){ t->f1c = 0x1d; return 6; }
