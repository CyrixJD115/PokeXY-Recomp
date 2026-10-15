/* DllHeading@0x8ac (size 0x44) -- masked match.
 * Style: trick(void-ptr-passthrough) -- callee declared void*, (char*)fn(p)-4 avoids a spare mov
 * Same as @0x868 but vtable from literal pool, returns the adjusted pointer.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllHeading 0x8ac 0x44 xy-decomp/armcc-matching/matched/DllHeading/0x8ac.c
 */
typedef struct Q { unsigned char pad[0x18]; void *h; } Q;
typedef struct X { void *vptr; unsigned char sub[0x8]; int f12; int f16; Q *q; } X;
extern void *VT[];
extern void fn1e0(void *h, int a);
extern void fn1e8(void *h, int a);
extern void *fn1f0(void *s);
void *Fn(X *o)
{
    void *h;
    o->vptr = VT;
    h = o->q->h;
    fn1e0(h, o->f12);
    fn1e8(h, o->f16);
    return (char *)fn1f0(o->sub) - 4;
}
