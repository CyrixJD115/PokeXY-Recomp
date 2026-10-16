/* DllField function @ CRO file offset 0x83ad0, size 0x5c -- MATCHED (masked, 23/23).
 * release stage: release member +12 through vtable slot 1 and zero it; if member +8 exists, run import fx(m,1) on it then (if still set) release through slot 1 and zero it
 * Style: natural -- plain null-check/release/zero sequence
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x83ad0 0x5c xy-decomp/armcc-matching/matched/DllField/0x83ad0.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[8]; O *b; O *a; } S;
extern void fx(O *, int);
void fn(S *s)
{
    if (s->a) {
        s->a->vt->s1(s->a);
        s->a = 0;
    }
    if (s->b) {
        fx(s->b, 1);
        if (s->b) {
            s->b->vt->s1(s->b);
            s->b = 0;
        }
    }
}
