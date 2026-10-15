/* DllTownmap function @ CRO file offset 0x5ac, size 0x54 - MATCHED (masked, 21/21).
 * Style: natural
 * release helper: virtual-delete member [0x14] (vtbl slot 6) and [0x18] (slot 1) if set, zeroing each;
 * then call fn on this+0x1c and return this->[0xc].
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x5ac 0x54 xy-decomp/armcc-matching/matched/DllTownmap/0x5ac.c
 */

struct Obj { void **vtbl; };
extern void fn1b8(void *p);
int f(char *this)
{
    struct Obj *a = *(struct Obj **)(this + 0x14);
    if (a) {
        ((void (*)(struct Obj *))a->vtbl[6])(a);
        *(struct Obj **)(this + 0x14) = 0;
    }
    {
        struct Obj *b = *(struct Obj **)(this + 0x18);
        if (b) {
            ((void (*)(struct Obj *))b->vtbl[1])(b);
            *(struct Obj **)(this + 0x18) = 0;
        }
    }
    fn1b8(this + 0x1c);
    return *(int *)(this + 0xc);
}
