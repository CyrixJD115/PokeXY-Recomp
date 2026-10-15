/*
 * DllPhotoViewer @ 0x1974 (size 0x34) -- masked match 13/13.
 * dtor: install vtable, free buf at +0x14 if non-null, zero it, return this
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPhotoViewer 0x1974 0x34 xy-decomp/armcc-matching/matched/DllPhotoViewer/0x1974.c
 */
extern void f288(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void *fn(O *o){
    void *p = o->buf;
    o->vptr = VT;
    if (p) { f288(p); o->buf = 0; }
    return o;
}
