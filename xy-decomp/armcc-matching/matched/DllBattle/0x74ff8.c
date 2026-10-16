/* DllBattle function @ CRO file offset 0x74ff8, size 0x58 -- MATCHED (masked, 22/22).
 * Gated 3-call sequence on the handle at this+0xd38: if f18e4(h,0,1) succeeds then
 * f18ec(h,0,1,0); f34c(h,0,0,0); return 1; else return 0. Sibling of 0x75050.
 * Style: trick(redundant-local) -- the 2nd call's handle comes through a pointer local to the
 *   member (void **pp = &o->fld), reproducing retail's r5 = &fld; the other two use o->fld.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x74ff8 0x58 xy-decomp/armcc-matching/matched/DllBattle/0x74ff8.c
 */
struct O { char pad[0xd38]; void *fld; };
extern int f18e4(void *p, int a, int b);
extern void f18ec(void *p, int a, int b, int c);
extern void f34c(void *p, int a, int b, int c);

int seq(struct O *o)
{
    void **pp = &o->fld;
    if (f18e4(o->fld, 0, 1)) {
        f18ec(o->fld, 0, 1, 0);
        f34c(*pp, 0, 0, 0);
        return 1;
    }
    return 0;
}
