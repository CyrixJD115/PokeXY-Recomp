/* DllBattle function @ CRO file offset 0x75050, size 0x54 -- MATCHED (masked, 21/21).
 * Three-call sequence on the sub-object stored at this+0xd38 (a battle UI/ctrl handle):
 * f18ec(h,0,0,0); f34c(h,0,0,1); f18f4(h,0,1,1,1) -- the handle is reloaded from the field before
 * each call (calls may change it). Siblings 0x74ff8 / 0x750a4 use the same this+0xd38 field.
 * Style: trick(redundant-local) -- the field is accessed through a pointer local to the member
 *   (void **pp = &o->fld) because retail forms &o->fld once (r4) but loads the first arg via r0;
 *   direct o->fld uses give 2/21.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x75050 0x54 xy-decomp/armcc-matching/matched/DllBattle/0x75050.c
 */
struct O { char pad[0xd38]; void *fld; };
extern void f18ec(void *p, int a, int b, int c);
extern void f34c(void *p, int a, int b, int c);
extern void f18f4(void *p, int a, int b, int c, int d);

void seq(struct O *o)
{
    void **pp = &o->fld;
    f18ec(*pp, 0, 0, 0);
    f34c(*pp, 0, 0, 1);
    f18f4(*pp, 0, 1, 1, 1);
}
