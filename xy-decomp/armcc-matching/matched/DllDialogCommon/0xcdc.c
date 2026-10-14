/* DllDialogCommon function @ CRO file offset 0xcdc, size 0x34 — MATCHED (masked, 13/13). Destructor: vptr install, free member @+0x14, returns this (ARM C++ ABI).
 *
 * Verify: python3 tools/fingerprint_fn.py DllDialogCommon 0xcdc 0x34 test/fingerprint/dialog_common_0xcdc.c
 */

/* 0xcdc: destructor without base-dtor tail call; returns this (ARM C++ ABI). */
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
