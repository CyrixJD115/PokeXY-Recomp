/*
 * DllGts @ 0x11a24, size 0x70, masked match (full words). tick counter f44; at 0x50 sets f50=0x58, calls fn, deletes ctx->f9c via vtable slot 1; returns 0x18/0x19.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x11a24 0x70 xy-decomp/armcc-matching/matched/DllGts/0x11a24.c
 */

typedef struct V { void *s0; void (*d)(void*); } V;
typedef struct Ob { V *vt; } Ob;
typedef struct C { unsigned char p[0x9c]; Ob *f9c; } C;
typedef struct O { unsigned char p1[0x24]; C *f24; unsigned char p2[0x1c]; unsigned f44; unsigned char p3[0x8]; int f50; } O;
extern void fc(void*,int); extern void fd(void*);
int F(O *t){
  unsigned n = t->f44 + 1;
  t->f44 = n;
  if (n < 0x50) return 0x18;
  t->f50 = 0x58;
  fc(t->f24, 1);
  { C *c = t->f24;
    if (c->f9c) {
      fd(c->f9c);
      if (c->f9c) c->f9c->vt->d(c->f9c);
      c->f9c = 0;
    }
  }
  return 0x19;
}
