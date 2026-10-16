/* DllField function @ CRO file offset 0x1209c, size 0x58 -- MATCHED (masked, 22/22).
 * message forwarder to child at this+0x80: msgs 16/56 -> vtable slot 0, msg 22 -> slot 1, return 0; else return 1
 * Style: natural -- plain switch
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x1209c 0x58 xy-decomp/armcc-matching/matched/DllField/0x1209c.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); void (*s2)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[128]; O *o; } S;
int fn(S *s, int msg)
{
    O *o = s->o;
    if (o) {
        switch (msg) {
        case 16:
        case 56:
            o->vt->s0(o);
            return 0;
        case 22:
            o->vt->s1(o);
            return 0;
        }
    }
    return 1;
}
