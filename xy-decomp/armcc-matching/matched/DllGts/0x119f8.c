/*
 * DllGts @ 0x119f8, size 0x18, masked match (full words). fn(this->f24,1); return 3.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x119f8 0x18 xy-decomp/armcc-matching/matched/DllGts/0x119f8.c
 */

typedef struct O { unsigned char p1[0x24]; void *f24; } O;
extern void fc(void*,int);
int F(O *t){ fc(t->f24,1); return 3; }
