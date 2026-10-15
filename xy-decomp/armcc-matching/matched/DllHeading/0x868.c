/* DllHeading@0x868 (size 0x40) -- masked match.
 * Style: trick(void-ptr-passthrough) -- callee returns void*, result offset (char*)fn(p)-4 then tail-called
 * Stores passed-in vtable, forwards two field values to the handle at o->q->h, then tail-calls with the adjusted base pointer.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllHeading 0x868 0x40 xy-decomp/armcc-matching/matched/DllHeading/0x868.c
 */
typedef struct Q { unsigned char pad[0x18]; void *h; } Q;
typedef struct X { void *vptr; unsigned char sub[0x8]; int f12; int f16; Q *q; } X;
extern void fn1e0(void *h, int a);
extern void fn1e8(void *h, int a);
extern void *fn1f0(void *s);
extern void *fn1f8(void *s);
void *Fn(X *o, void *vt)
{
    void *h;
    o->vptr = vt;
    h = o->q->h;
    fn1e0(h, o->f12);
    fn1e8(h, o->f16);
    return fn1f8((char *)fn1f0(o->sub) - 4);
}
