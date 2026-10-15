/*
 * DllGts @ 0xb0f4 (size 0x18) -- masked match 6/6.
 * Sets this->byte@0x40 = 1 and clears this->ctx@0x24->byte@0x624.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0xb0f4 0x18 xy-decomp/armcc-matching/matched/DllGts/0xb0f4.c
 */
typedef struct S { unsigned char p[1572]; unsigned char f; } S;
typedef struct O { unsigned char p[36]; S *f24; unsigned char q[24]; unsigned char f40; } O;
void F(O *t){ t->f40 = 1; t->f24->f = 0; }
