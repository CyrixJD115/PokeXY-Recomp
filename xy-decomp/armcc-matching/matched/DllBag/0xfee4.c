/* DllBag function @ CRO file offset 0xfee4, size 0x24 -- MATCHED (masked, 9/9).
 * calls fn(this->c->p), fn2(this), returns 40
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0xfee4 0x24 xy-decomp/armcc-matching/matched/DllBag/0xfee4.c
 */
extern void fnA(void *x);
extern void fnB(void *t);
struct In { void *p; };
struct S { unsigned char pad[0x44]; struct In *c; };
int f(struct S *this) { fnA(this->c->p); fnB(this); return 40; }
