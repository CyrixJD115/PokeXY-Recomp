/* DllHeading@0x1cec (size 0x44) -- masked match.
 * Style: trick(void-ptr-passthrough) -- callee declared void*, (char*)fn(p)-4 avoids a spare mov
 * Sibling of @0x1738: installs its own vtable (literal pool), pushes the two config ints (this+16, this+20) into the handle at this->q->h (q at +12, handle at q+0x18), returns the sub-object (this+4) base adjusted by -4.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllHeading 0x1cec 0x44 xy-decomp/armcc-matching/matched/DllHeading/0x1cec.c
 */
extern void *VT[];
typedef struct Q { unsigned char pad[0x18]; void *h; } Q;
typedef struct X { void *vptr; unsigned char sub[0x8]; Q *q; int fa; int fb; } X;
extern void fn1e0(void *h, int a);
extern void fn1e8(void *h, int a);
extern void *fn1f0(void *s);
void *Fn(X *o)
{
    void *h;
    o->vptr = VT;
    h = o->q->h;
    fn1e0(h, o->fa);
    fn1e8(h, o->fb);
    return (char *)fn1f0(o->sub) - 4;
}
