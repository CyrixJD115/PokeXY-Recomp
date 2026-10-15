/* DllTownmap function @ CRO file offset 0xd68, size 0x1c - MATCHED (masked, 7/7).
 * Style: trick(extra-arg) -- two forwarded int args shift the vtable load into r3 (retail slot-3 wrapper takes 2 forwarded args)
 * virtual-call wrapper: if (this->[0x88]) tail-call its vtable slot 3 passing through args.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0xd68 0x1c xy-decomp/armcc-matching/matched/DllTownmap/0xd68.c
 */

struct Obj { void **vtbl; };
void f(char *this, int a, int b)
{
    struct Obj *o = *(struct Obj **)(this + 0x88);
    if (o) ((void (*)(struct Obj *, int, int))o->vtbl[3])(o, a, b);
}
