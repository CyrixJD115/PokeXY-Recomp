/* DllField function @ CRO file offset 0xe7264, size 0x54 -- MATCHED (masked, 21/21).
 * message forwarder to child at this+0x10: msg 28 -> vtable slot 0, msgs 34/60 -> slot 1, return 0; else return 1
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xe7264 0x54 xy-decomp/armcc-matching/matched/DllField/0xe7264.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); void (*s2)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[16]; O *o; } S;
int fn(S *s, int msg)
{
    O *o = s->o;
    if (o) {
        if (msg == 28) {
            o->vt->s0(o);
            return 0;
        }
        if (msg == 34 || msg == 60) {
            o->vt->s1(o);
            return 0;
        }
    }
    return 1;
}
