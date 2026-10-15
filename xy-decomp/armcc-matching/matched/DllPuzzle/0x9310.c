/* DllPuzzle @ 0x9310 size 0x2c MATCHED (masked 11/11): dtor stage: install vtable, call fn(o->p12, o->p4->f34), return this.
 * 
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x9310 0x2c xy-decomp/armcc-matching/matched/DllPuzzle/0x9310.c
 */
typedef struct C { void *vptr; struct { unsigned char pad[0x34]; void *f34; } *p4; unsigned char pad[4]; void *p12; } C;
extern void *VT[];
extern void fn8d0(void *a, void *b);
C *f(C *o)
{
    o->vptr = VT;
    fn8d0(o->p12, o->p4->f34);
    return o;
}
