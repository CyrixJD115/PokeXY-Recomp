/*
 * DllGts @ 0x11a10, size 0x14, masked match (full words). *this->f10=1; this->byte@0x40=1.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x11a10 0x14 xy-decomp/armcc-matching/matched/DllGts/0x11a10.c
 */

typedef struct O { unsigned char p0[0x10]; unsigned char *f10; unsigned char p1[0x2c]; unsigned char f40; } O;
void F(O *t){ *t->f10 = 1; t->f40 = 1; }
