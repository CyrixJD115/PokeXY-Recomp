/* DllField function @ CRO file offset 0x3cdb4, size 0x50 -- MATCHED (masked, 20/20).
 * dtor stage: install vtable, release member +4 through its vtable slot 1 and zero it, free import-owned member +8 and zero it, return this
 * Style: trick(ordering-local) -- member/vtable locals only to fix load / pool-load order (member load before vtable pool-load)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x3cdb4 0x50 xy-decomp/armcc-matching/matched/DllField/0x3cdb4.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { void *vt; O *a; void *b; } S;
extern void *VT[];
extern void fre(void *);
S *fn(S *s)
{
    O *a = s->a;
    void *vt = VT;
    s->vt = vt;
    if (a) {
        a->vt->s1(a);
        s->a = 0;
    }
    if (s->b) {
        fre(s->b);
        s->b = 0;
    }
    return s;
}
