/*
 * DllGts @ 0x7c74, size 0x18, masked match 6/6.
 * Calls an import with (this->child@0x20, 1) and returns 4 (a state/step code).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x7c74 0x18 xy-decomp/armcc-matching/matched/DllGts/0x7c74.c
 */
typedef struct O { unsigned char p[0x20]; void *c; } O;
extern void fn(void *, int);
int F(O *t){ fn(t->c, 1); return 4; }
