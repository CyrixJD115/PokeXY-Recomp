/*
 * DllGts @ 0x16aec (size 0x14) -- masked match 5/5.
 * Setter: sets byte *(this->ptr@0x0c) = 1 and this->byte@0x3c = 1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0x16aec 0x14 xy-decomp/armcc-matching/matched/DllGts/0x16aec.c
 */
typedef struct O { unsigned char p[12]; unsigned char *f0c; unsigned char q[44]; unsigned char f3c; } O;
void F(O *t){ t->f0c[0] = 1; t->f3c = 1; }
