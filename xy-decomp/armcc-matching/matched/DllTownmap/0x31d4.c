/*
 * DllTownmap @ 0x31d4 (size 0x38) -- masked match 14/14.
 * dtor-stage: install vtable, free buf at +0x14 if non-null, zero it, tail-call base f230(this)
 * Style: trick(ordering-local) -- local vt = VT forces the vtable pool load ahead of the member load
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x31d4 0x38 xy-decomp/armcc-matching/matched/DllTownmap/0x31d4.c
 */
extern void f328(void *);
extern void *f230(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void *fn(O *o){
    void *vt = VT;
    void *p = o->buf;
    o->vptr = vt;
    if (p) { f328(p); o->buf = 0; }
    return f230(o);
}
