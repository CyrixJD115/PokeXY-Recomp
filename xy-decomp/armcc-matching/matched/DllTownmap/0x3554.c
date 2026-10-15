/* DllTownmap function @ CRO file offset 0x3554, size 0x1c - MATCHED (masked, 7/7).
 * Style: trick(extra-arg) -- unused pass-through int arg(s) added only to shift register allocation
 * virtual-call wrapper: if (this->[0x10]) call its vtable slot 4 passing through arg1 (tail call).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x3554 0x1c xy-decomp/armcc-matching/matched/DllTownmap/0x3554.c
 */

struct Obj { void **vtbl; };
void vcall_member(char *this, int a)
{
    struct Obj *o = *(struct Obj **)(this + 0x10);
    if (o) ((void (*)(struct Obj *, int))o->vtbl[4])(o, a);
}
