/* DllBag function @ CRO file offset 0x10334, size 0x2c -- MATCHED (masked, 11/11).
 * fnA(this); fnB(this->ctx->p0->f8,2,3,1); returns 71
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x10334 0x2c xy-decomp/armcc-matching/matched/DllBag/0x10334.c
 */
extern void fnA(void *t);
extern void fnB(void *x, int a, int b, int c);
struct Dd { unsigned char pad[8]; void *f8; };
struct E { struct Dd *d; };
struct S { unsigned char pad[0x4c]; struct E *c; };
int f(struct S *this)
{
    struct E *c = this->c;
    fnA(this);
    fnB(c->d->f8, 2, 3, 1);
    return 71;
}
