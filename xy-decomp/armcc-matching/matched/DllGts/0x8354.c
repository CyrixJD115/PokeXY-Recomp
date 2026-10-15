/*
 * DllGts @ 0x8354, size 0x3c, masked match 15/15.
 * Sibling of 0x824c: same import call with (&this->f24, 1, &state->f250,
 * state->int@0x21c, 0) and returns next-state code 13.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x8354 0x3c xy-decomp/armcc-matching/matched/DllGts/0x8354.c
 */
typedef struct B { unsigned char p[0x21c]; int f; unsigned char q[0x30]; unsigned char g[4]; } B;
typedef struct O { unsigned char p0[12]; B *b; unsigned char q[0x10]; void *c; unsigned char s[4]; } O;
extern void fn(void *, void *, int, void *, int, int);
int F(O *t){ fn(t->c, t->s, 1, t->b->g, t->b->f, 0); return 13; }
