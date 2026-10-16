/*
 * DllBattle @ 0x8db3c, size 0x28 -- MATCHED (masked, 10/10).
 * If the object at this+216 is non-null: release/notify it (fa(m)), then forward the second
 * argument to fb(a) as a tail call; otherwise do nothing (void).
 * Family (same bytes, only the member offset differs): 0x8b2e8(+232), 0x8bcb8(+236), 0x8e858(+232), 0x8f210(+224), 0x8fa38(+224).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x8db3c 0x28 xy-decomp/armcc-matching/matched/DllBattle/0x8db3c.c
 */

struct O { char pad[216]; void *m; };
extern void fa(void *);
extern void fb(void *);

void f(struct O *o, void *a)
{
    if (o->m) {
        fa(o->m);
        fb(a);
    }
}
