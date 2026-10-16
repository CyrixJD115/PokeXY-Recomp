/* DllBox function @ CRO file offset 0x1dde8, size 0x48 -- MATCHED (masked, 18/18).
 * Init helper: copies arg->f into the owned sub-object, calls two import
 * veneers on it, stores results, and stores (*(f->[28])) + 0x13650. Returns 1.
 * Note: load arg->f into a pointer-typed local ONCE and reuse it; two
 * separate (int) reads give a wrong register schedule.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x1dde8 0x48 xy-decomp/armcc-matching/matched/DllBox/0x1dde8.c
 */

extern void *fn1(void *);
extern void *fn2(void *);
struct In { int pad[4]; void *a; int b; int c; int d; };
struct Ctx { int pad[9]; struct In *in; };
struct Arg { int pad[5]; void *f; };
int setup(struct Ctx *self, struct Arg *arg)
{
    struct In *s = self->in;
    int *p = (int *)((char *)s + 4);
    void *f = arg->f;
    s->a = f;
    p[4] = (int)fn1(f);
    p[5] = (int)fn2((void *)p[3]);
    p[6] = **(int **)((char *)p[3] + 28) + 0x13650;
    return 1;
}
