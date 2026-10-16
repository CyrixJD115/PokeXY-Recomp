/* DllField function @ CRO file offset 0x412e4, size 0x5c -- MATCHED (masked, 23/23).
 * release stage: free import-owned member +0x34, release members +0x30 and +0x2c through vtable slot 1, zeroing each (byte-identical siblings)
 * Style: natural -- plain null-check/release/zero sequence
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x412e4 0x5c xy-decomp/armcc-matching/matched/DllField/0x412e4.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[44]; O *a; O *b; void *c; } S;
extern void fre(void *);
void fn(S *s)
{
    if (s->c) {
        fre(s->c);
        s->c = 0;
    }
    if (s->b) {
        s->b->vt->s1(s->b);
        s->b = 0;
    }
    if (s->a) {
        s->a->vt->s1(s->a);
        s->a = 0;
    }
}
