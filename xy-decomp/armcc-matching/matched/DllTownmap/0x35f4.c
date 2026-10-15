/* DllTownmap function @ CRO file offset 0x35f4, size 0x1c - virtual-call wrapper, slot 3 of member +0x10.
 * Style: trick(extra-arg) -- unused pass-through int arg(s) added only to shift register allocation
 * Needs two pass-through args so the vtable load lands in r3 (not r1/r2).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x35f4 0x1c xy-decomp/armcc-matching/matched/DllTownmap/0x35f4.c
 */
struct Obj { void **vtbl; };
void vcall_member(char *this, int a, int b)
{
    struct Obj *o = *(struct Obj **)(this + 0x10);
    if (o) ((void (*)(struct Obj *, int, int))o->vtbl[3])(o, a, b);
}
