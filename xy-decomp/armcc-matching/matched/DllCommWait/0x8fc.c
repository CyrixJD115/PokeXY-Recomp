/* DllCommWait function @ CRO file offset 0x8fc, size 0x20 -- MATCHED (masked).
 * `o = this->a->b; if (o) return o->vtbl[9](o); (void)`
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllCommWait 0x8fc 0x20 xy-decomp/armcc-matching/matched/DllCommWait/0x8fc.c
 */
typedef struct B B;
struct B { void **vt; };
typedef struct A { int pad; B *b; } A;
typedef struct T { unsigned char pad[36]; A *a; } T;

void F(T *t)
{
    B *o = t->a->b;
    if (o)
        ((void (*)(B *))o->vt[9])(o);
}
