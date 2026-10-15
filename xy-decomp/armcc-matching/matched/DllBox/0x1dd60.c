/* DllBox function @ CRO file offset 0x1dd60, size 0x88 -- MATCHED (masked, 34/34).
 * Teardown: this->in->p; if non-null and q->c != 1, virtual call (vtable slot 9)
 * on q->o, three f8c8 calls + one f8d0, then tail-call f8d8(p->x) if p->x non-null.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x1dd60 0x88 xy-decomp/armcc-matching/matched/DllBox/0x1dd60.c
 */

typedef struct VO { void **vt; } VO;
typedef struct Q {
    void *a0;
    void *a4;
    void *a8;
    unsigned char c;
    unsigned char pad[0xe2f];
    VO *o;
} Q;
typedef struct P { Q *q; unsigned char pad[0x518]; void *x; } P;
typedef struct In { unsigned char pad[0x50]; P *p; } In;
typedef struct S { unsigned char pad[0x24]; In *in; } S;
extern void f8c8(void *, void *, int);
extern void f8d0(void *, void *);
extern void f8d8(void *);
void run(S *self)
{
    P *p = self->in->p;
    if (p) {
        Q *q = p->q;
        if (q->c != 1) {
            ((void (*)(VO *))q->o->vt[9])(q->o);
            f8c8(q->a4, q->a0, 0);
            f8d0(q->a8, q->a0);
            f8c8(q->a4, q->a0, 1);
            f8c8(q->a4, q->a0, 2);
        }
        if (p->x)
            f8d8(p->x);
    }
}
