/* DllPuzzle @ 0x1438 size 0x38 MATCHED (masked 13/13 (+pool word)): dtor stage: install vtable, call base cleanup, free via obj->p4->f14 chain, return this.
 * Style: natural
 * 
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x1438 0x38 xy-decomp/armcc-matching/matched/DllPuzzle/0x1438.c
 */
typedef struct D { void *vptr; struct { unsigned char pad[0x14]; void *f14; } *p4; } D;
extern void *VT[];
extern void fn71b0(D *o);
extern void *fn1c8(void *p);
extern void fn260(void *p, int n);
D *f(D *o)
{
    o->vptr = VT;
    fn71b0(o);
    fn260(fn1c8(o->p4->f14), 17);
    return o;
}
