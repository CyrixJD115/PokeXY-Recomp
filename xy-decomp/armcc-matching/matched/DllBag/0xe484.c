/* DllBag function @ CRO file offset 0xe484, size 0x24 -- MATCHED (masked, 9/9).
 * veneer(this); this->sub(+8).field_0x30 = fnB(this); returns 0
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0xe484 0x24 xy-decomp/armcc-matching/matched/DllBag/0xe484.c
 */
extern void fnA(void *t);
extern int fnB(void *t);
struct Sub { unsigned char pad[0x30]; int v; };
struct S { unsigned char pad[8]; struct Sub sub; };
int f(struct S *this)
{
    struct Sub *s = &this->sub;
    fnA(this);
    s->v = fnB(this);
    return 0;
}
