/* DllDendouDemo function @ CRO file offset 0x34c4, size 0x1c -- MATCHED (masked).
 * `o = this->a->b; if (o) return fn(o->first); (void)`
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllDendouDemo 0x34c4 0x1c xy-decomp/armcc-matching/matched/DllDendouDemo/0x34c4.c
 */
extern void fn(int);
typedef struct A { unsigned char pad[40]; int *b; } A;
typedef struct T { unsigned char pad[36]; A *a; } T;

void F(T *t)
{
    int *o = t->a->b;
    if (o)
        fn(*o);
}
