/* DllField function @ CRO file offset 0x10c54, size 0x20 -- MATCHED (masked, 8/8).
 * flag setter, mirror of 0x10c34: sub[84]=1, sub[85]=0, this[73]=1
 * Style: trick(redundant-local) -- locals z=0,o=1 needed for retail register allocation (plain literals mismatch)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x10c54 0x20 xy-decomp/armcc-matching/matched/DllField/0x10c54.c
 */

struct T { char pad[84]; unsigned char a, b; };
struct S { char pad[68]; struct T *t; char p2[1]; unsigned char f; };

void set_flags(struct S *s)
{
    unsigned char z = 0, o = 1;
    s->t->a = o;
    s->t->b = z;
    s->f = o;
}
