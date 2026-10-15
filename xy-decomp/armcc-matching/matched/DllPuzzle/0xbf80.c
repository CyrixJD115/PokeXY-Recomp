/*
 * DllPuzzle @ 0xbf80 (size 0x38) -- masked match 14/14.
 * dtor-stage: install vtable, free buf at +0x14 if non-null, zero it, tail-call base f268(this)
 * Style: trick(ordering-local) -- local vt = VT forces the vtable pool load ahead of the member load
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPuzzle 0xbf80 0x38 xy-decomp/armcc-matching/matched/DllPuzzle/0xbf80.c
 */
extern void f1f8(void *);
extern void *f268(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
void *fn(O *o){
    void *vt = VT;
    void *p = o->buf;
    o->vptr = vt;
    if (p) { f1f8(p); o->buf = 0; }
    return f268(o);
}
