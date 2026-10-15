/* DllPuzzle @ 0x92e4 size 0x28 MATCHED (masked): dtor-stage variant: store vt arg, fn8d0(o->p12,o->p4->f34), tail-call fn268(o).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x92e4 0x28 xy-decomp/armcc-matching/matched/DllPuzzle/0x92e4.c
 */
typedef struct C { void *vptr; struct { unsigned char pad[0x34]; void *f34; } *p4; unsigned char pad[4]; void *p12; } C;
extern void fn8d0(void *a, void *b);
extern void *fn268(void *p);
void *f(C *o, void *vt)
{
    o->vptr = vt;
    fn8d0(o->p12, o->p4->f34);
    return fn268(o);
}
