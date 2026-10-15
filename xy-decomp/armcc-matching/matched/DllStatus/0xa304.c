/*
 * DllStatus @ 0xa304 (size 0x38) -- masked match 14/14.
 * dtor-stage: install vtable, free buf at +0x14 if non-null, zero it, tail-call base f310(this)
 * Style: trick(ordering-local) -- local vt = VT forces the vtable pool load ahead of the member load
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xa304 0x38 xy-decomp/armcc-matching/matched/DllStatus/0xa304.c
 */
extern void f968(void *);
extern void *f310(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void *fn(O *o){
    void *vt = VT;
    void *p = o->buf;
    o->vptr = vt;
    if (p) { f968(p); o->buf = 0; }
    return f310(o);
}
