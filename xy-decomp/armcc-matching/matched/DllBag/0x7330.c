/* DllBag function @ CRO file offset 0x7330, size 0x50 -- MATCHED (masked, 20/20).
 * same as DllBag@0xc508 with q->b at +1 and test b==0
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x7330 0x50 xy-decomp/armcc-matching/matched/DllBag/0x7330.c
 */
extern void *fnA(void *t);
extern void fnB(void *t);
struct P { unsigned char pad[0x42]; unsigned short h; };
struct Q { unsigned char pad[1]; unsigned char b; };
struct Sub { struct P *p; unsigned char pad[0x2c]; void *v; };
struct S { unsigned char pad[8]; struct Sub sub; unsigned char pad2[0x44 - 0x8 - 0x34]; struct Q *q; };
int f(struct S *this)
{
    struct Sub *s = &this->sub;
    int r;
    if (this->q->b == 0) {
        s->v = fnA(this);
        r = 0;
    } else {
        s->p->h = 1;
        r = 75;
    }
    fnB(this);
    return r;
}
