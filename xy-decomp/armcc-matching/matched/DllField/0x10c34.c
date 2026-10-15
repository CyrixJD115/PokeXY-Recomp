/* DllField function @ CRO file offset 0x10c34, size 0x20 -- MATCHED (masked, 8/8).
 * flag setter: this->sub->[84]=0, this->sub->[85]=1 (sub = *(this+68), reloaded), this[73]=1.
 * Style: trick(redundant-local) -- Without the z/o locals armcc hoists the sub-pointer into r2 and allocates the constants differently (5-vs-8 word mismatch); the locals `z=0,o=1` reproduce retail's register choice (r3 for pointer, r2=0, r1=1).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x10c34 0x20 xy-decomp/armcc-matching/matched/DllField/0x10c34.c
 */

struct T { char pad[84]; unsigned char a, b; };
struct S { char pad[68]; struct T *t; char p2[1]; unsigned char f; };

void set_flags(struct S *s)
{
    unsigned char z = 0, o = 1;
    s->t->a = z;
    s->t->b = o;
    s->f = o;
}
