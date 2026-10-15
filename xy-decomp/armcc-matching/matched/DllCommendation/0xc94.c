/* DllCommendation function @ CRO file offset 0xc94, size 0x38 — MATCHED (masked, 14/14). First match in this module (eighth module overall for the technique).
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 *
 * Same vptr-install/free-member destructor idiom already documented for
 * DllFatalErrorProc@0x284 and DllDialogCommon@0xcdc: install a vtable
 * pointer, free one owned member at +0x14 via a masked call if non-null,
 * zero it, and instead of returning `this` directly it tail-calls a
 * second import with `this` already sitting in r0 (`pop {r4, lr}; b
 * <import>`, not `bl`+`pop {r4,pc}` — lr is popped before the branch, a
 * true tail call).
 *
 * Non-obvious part: register/instruction order, identical to 0x284's
 * "good" shape per the toolchain's residue-class rule #4 — the vtable
 * literal is loaded into r1 BEFORE the member load into r0, and the
 * store to o->vptr is deferred until after both loads. Writing `vt =
 * VT;` as its own local computed before `p = o->buf;`, then `o->vptr =
 * vt;` right after, reproduced this scheduling on the first try.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllCommendation 0xc94 0x38 xy-decomp/armcc-matching/matched/DllCommendation/0xc94.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free_c94(void *p);
extern X *finish_c94(X *o);  /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free_c94(p);
        o->buf = 0;
    }
    return finish_c94(o);
}
