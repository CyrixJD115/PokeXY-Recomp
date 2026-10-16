/*
 * Style: natural
 * DllGts @ 0x453c, size 0x10, masked match 4/4.
 * Clears this->word@0x5c, returns 0x28.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x453c 0x10 xy-decomp/armcc-matching/matched/DllGts/0x453c.c
 */

typedef struct O { unsigned char p[0x5c]; int f5c; } O;
int F(O *t){ t->f5c = 0; return 0x28; }
