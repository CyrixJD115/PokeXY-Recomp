/*
 * Style: natural
 * DllGts @ 0x119b4, size 0x44, masked match (full words). 3 calls on this->f24 (args 0x81,1,f50 / 6 / 1), clear byte 0x41, return 0x1a.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x119b4 0x44 xy-decomp/armcc-matching/matched/DllGts/0x119b4.c
 */

typedef struct O { unsigned char p0[0x10]; unsigned char *f10; unsigned char p1[0x10]; void *f24; unsigned char p2[0x18]; unsigned char f40; unsigned char f41; unsigned char p3[0xe]; int f50; } O;
extern void fa(void*,int,int,int); extern void fb(void*,int); extern void fc(void*,int);
int F(O *t){ fa(t->f24,0x81,1,t->f50); fb(t->f24,6); fc(t->f24,1); t->f41=0; return 0x1a; }
