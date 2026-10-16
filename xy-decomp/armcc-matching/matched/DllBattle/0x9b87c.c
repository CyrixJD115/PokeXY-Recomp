/*
 * DllBattle @ 0x9b87c, size 0x24 -- MATCHED (masked, 9/9).
 * Variant of the 0x8b2e8 family with no second argument: if the object at this+224 is non-null,
 * release/notify it (fa(m)), then tail-call fb(0); otherwise do nothing (void).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x9b87c 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x9b87c.c
 */

struct O { char pad[224]; void *m; };
extern void fa(void *);
extern void fb(void *);

void f(struct O *o)
{
    if (o->m) {
        fa(o->m);
        fb(0);
    }
}
