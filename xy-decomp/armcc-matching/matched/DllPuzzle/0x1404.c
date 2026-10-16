/* DllPuzzle @ 0x1404 size 0x30 MATCHED (masked): dtor-stage variant: store vt arg, call base cleanup, fn260(fn1c8(o->p4->f14),17), tail-call fn268(o).
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x1404 0x30 xy-decomp/armcc-matching/matched/DllPuzzle/0x1404.c
 */
typedef struct D { void *vptr; struct { unsigned char pad[0x14]; void *f14; } *p4; } D;
extern void fn71b0(D *o);
extern void *fn1c8(void *p);
extern void fn260(void *p, int n);
extern void *fn268(void *p);
void *f(D *o, void *vt)
{
    o->vptr = vt;
    fn71b0(o);
    fn260(fn1c8(o->p4->f14), 17);
    return fn268(o);
}
