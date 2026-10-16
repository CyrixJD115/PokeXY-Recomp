/* DllField function @ CRO file offset 0x151f4, size 0x54 -- MATCHED (masked, 21/21).
 * Message forwarder: if the child object at this+0x84 exists, msg 28 calls its vtable slot 0 and
 * msgs 34/60 call slot 1 (offset 4), both returning 0 (handled); every other case returns 1.
 * Sibling of 0xe701c (offset +4, msgs 2/11).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x151f4 0x54 xy-decomp/armcc-matching/matched/DllField/0x151f4.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); void (*s2)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[132]; O *o; } S;
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
