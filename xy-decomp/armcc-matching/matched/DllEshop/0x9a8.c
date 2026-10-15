/*
 * DllEshop @ 0x9a8 (size 0x10) -- masked match 4/4.
 * Sets this->byte@0xe4 = 1 and returns 1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllEshop 0x9a8 0x10 xy-decomp/armcc-matching/matched/DllEshop/0x9a8.c
 */
typedef struct O { unsigned char p[228]; unsigned char fe4; } O;
int F(O *t){ t->fe4 = 1; return 1; }
