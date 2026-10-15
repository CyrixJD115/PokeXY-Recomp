/*
 * DllBag @ 0x13848 (size 0x38) -- masked match 14/14.
 * dtor-stage: install vtable, free buf at +0x14 if non-null, zero it, tail-call base dtor f300(this)
 * Style: trick(ordering-local) -- local `vt = VT` only forces the vtable pool load before the member load
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x13848 0x38 xy-decomp/armcc-matching/matched/DllBag/0x13848.c
 */
extern void fc60(void *);
extern void f300(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void fn(O *o){
    void *vt = VT;
    void *p = o->buf;
    o->vptr = vt;
    if (p) { fc60(p); o->buf = 0; }
    f300(o);
}
