/*
 * DllGts @ 0x824c, size 0x3c, masked match 15/15.
 * Calls an import on this->child@0x20 with (&this->f24, 0, &state->f250,
 * state->int@0x218, 0) (state = this->@0xc; 5th/6th args spill to the stack,
 * armcc packs them with strd) and returns next-state code 5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x824c 0x3c xy-decomp/armcc-matching/matched/DllGts/0x824c.c
 */
typedef struct B { unsigned char p[0x218]; int f; unsigned char q[0x34]; unsigned char g[4]; } B;
typedef struct O { unsigned char p0[12]; B *b; unsigned char q[0x10]; void *c; unsigned char s[4]; } O;
extern void fn(void *, void *, int, void *, int, int);
int F(O *t){ fn(t->c, t->s, 0, t->b->g, t->b->f, 0); return 5; }
