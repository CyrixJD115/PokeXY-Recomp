/*
 * DllGts @ 0xb054 (size 0x14) -- masked match 5/5.
 * Sets this->ctx@0x24->byte@0x624 = 1 and returns 5.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0xb054 0x14 xy-decomp/armcc-matching/matched/DllGts/0xb054.c
 */
typedef struct S { unsigned char p[1572]; unsigned char f; } S;
typedef struct O { unsigned char p[36]; S *f24; } O;
int F(O *t){ t->f24->f = 1; return 5; }
