/* DllSparringMenu function @ CRO file offset 0x5ad8, size 0x34 — MATCHED
 * (masked, 13/13). Destructor: vptr install, free member @+0x14, returns
 * `this` (ARM C++ ABI) — same idiom already proven in DllDialogCommon@0xcdc
 * and many other modules (member-load-before-vtable-load scheduling order).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSparringMenu 0x5ad8 0x34 \
 *     xy-decomp/armcc-matching/matched/DllSparringMenu/0x5ad8.c
 */
typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free218(void *p);

X *dtor(X *o)
{
    void *p = o->buf;
    o->vptr = VT;
    if (p) {
        free218(p);
        o->buf = 0;
    }
    return o;
}
