/* DllHeading@0x1738 (size 0x44) -- masked match.
 * Style: trick(void-ptr-passthrough) -- callee declared void*, (char*)fn(p)-4 avoids a spare mov
 * Sibling of @0x868/@0x8ac family: vtable from literal pool; forwards two child fields to handle o->q->h, returns/tail-calls with base adjusted by -4.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllHeading 0x1738 0x44 xy-decomp/armcc-matching/matched/DllHeading/0x1738.c
 */
typedef struct Q { unsigned char pad[0x18]; void *h; } Q;
typedef struct X { void *vptr; unsigned char sub[0x8]; Q *q; int fa; int fb; } X;
extern void *VT[];
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
