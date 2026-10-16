/* DllBattle function @ CRO file offset 0x750a4, size 0x84 -- MATCHED (masked, 33/33).
 * Operates on the handle at this+0xd38: f34c(h,0,0,1); f18f4(h,0,0,1,1); then looks up entry
 * `i` of an 8-byte {int x, int y} global table, converts both ints to float into a vec3 (z unset,
 * the 3rd slot is what puts the array at sp+4) and calls f18fc(h, 0, 2, vec). Size includes the
 * trailing pool word (table address). Sibling of 0x75050 / 0x74ff8.
 * Style: trick(redundant-local) -- `void **pp = &o->fld` caches the member address as retail does
 *   (first call loads o->fld directly, later ones go through *pp).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x750a4 0x84 xy-decomp/armcc-matching/matched/DllBattle/0x750a4.c
 */
struct O { char pad[0xd38]; void *fld; };
struct P { int x; int y; };
extern void f34c(void *p, int a, int b, int c);
extern void f18f4(void *p, int a, int b, int c, int d);
extern void f18fc(void *p, int a, int b, float *v);
extern struct P tab[];

void seq(struct O *o, int i)
{
    void **pp = &o->fld;
    float v[3];
    const struct P *e;
    f34c(o->fld, 0, 0, 1);
    f18f4(*pp, 0, 0, 1, 1);
    e = &tab[i];
    v[0] = (float)e->x;
    v[1] = (float)e->y;
    f18fc(*pp, 0, 2, v);
}
