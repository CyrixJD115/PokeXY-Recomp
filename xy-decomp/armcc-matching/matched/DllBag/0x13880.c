/*
 * DllBag @ 0x13880 (size 0x34) -- masked match 13/13.
 * dtor: install vtable, free owned buf at +0x14 if non-null and zero it, return this
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x13880 0x34 xy-decomp/armcc-matching/matched/DllBag/0x13880.c
 */
extern void fc60(void *);
extern char VT[];
typedef struct { void *vptr; char pad[16]; void *buf; } O;
O *fn(O *o){
    void *p = o->buf; o->vptr = VT;
    if (p) { fc60(p); o->buf = 0; }
    return o;
}
