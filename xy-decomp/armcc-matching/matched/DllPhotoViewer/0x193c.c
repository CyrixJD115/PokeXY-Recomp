/*
 * DllPhotoViewer @ 0x193c (size 0x38) -- masked match 14/14.
 * dtor-stage: install vtable, free buf at +0x14 if non-null, zero it, tail-call base f218(this)
 * Style: trick(ordering-local) -- local vt = VT forces the vtable pool load ahead of the member load
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPhotoViewer 0x193c 0x38 xy-decomp/armcc-matching/matched/DllPhotoViewer/0x193c.c
 */
extern void f288(void *);
extern void *f218(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void *fn(O *o){
    void *vt = VT;
    void *p = o->buf;
    o->vptr = vt;
    if (p) { f288(p); o->buf = 0; }
    return f218(o);
}
