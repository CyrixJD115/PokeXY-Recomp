/* DllBag function @ CRO file offset 0x102f8, size 0x3c -- MATCHED (masked, 15/15).
 * if (!fnT(this->c->p->f8, 1, 97)) return 18; else tail-call fnU(this)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x102f8 0x3c xy-decomp/armcc-matching/matched/DllBag/0x102f8.c
 */
extern int fnT(void *x, int a, int b);
extern int fnU(void *t);
struct A { unsigned char pad[8]; void *f8; };
struct B { struct A *p; };
struct S { unsigned char pad[0x4c]; struct B *c; };
int f(struct S *this)
{
    if (!fnT(this->c->p->f8, 1, 97)) return 18;
    return fnU(this);
}
