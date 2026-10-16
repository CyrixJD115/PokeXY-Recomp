/*
 * DllBattle @ 0x90590, size 0x28 -- MATCHED (masked, 10/10).
 * Sibling of the 0x8b2e8 family. If the object at this+224 is non-null: release/notify it (fa(m)), then forward the second
 * argument to fb(a) as a tail call; otherwise do nothing (void).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x90590 0x28 xy-decomp/armcc-matching/matched/DllBattle/0x90590.c
 */

struct O { char pad[224]; void *m; };
extern void fa(void *);
extern void fb(void *);

void f(struct O *o, void *a)
{
    if (o->m) {
        fa(o->m);
        fb(a);
    }
}
