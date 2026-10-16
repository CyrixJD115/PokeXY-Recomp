/* DllField function @ CRO file offset 0xe701c, size 0x50 -- MATCHED (masked, 20/20).
 * Message forwarder: if the child object at this+4 exists, msg 2 calls its vtable slot 0 and
 * msg 11 calls slot 2 (offset 8), both returning 0 (handled); every other case returns 1.
 * Same shape as DllNumberInput@0x1890-style dispatchers; sibling 0x151f4 (+0x84, msgs 28/34/60).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xe701c 0x50 xy-decomp/armcc-matching/matched/DllField/0xe701c.c
 */
struct O;
typedef struct V { void (*s0)(struct O *); void (*s1)(struct O *); void (*s2)(struct O *); } V;
typedef struct O { V *vt; } O;
typedef struct S { char pad[4]; O *o; } S;
int fn(S *s, int msg)
{
    O *o = s->o;
    if (o) {
        if (msg == 2) {
            o->vt->s0(o);
            return 0;
        }
        if (msg == 11) {
            o->vt->s2(o);
            return 0;
        }
    }
    return 1;
}
