/* DllTrialDownload function @ CRO file offset 0x728, size 0x38 — MATCHED
 * (masked, 14/14). First match in this module. Same vptr-install/
 * free-member destructor idiom already proven in DllFatalErrorProc@0x284/
 * DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/
 * DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764:
 * installs a new vtable, frees an owned member at +0x14 if non-null, then
 * tail-calls a second import instead of returning `this` directly.
 *
 * Load order matches the "good" scheduling class: the vtable literal is
 * loaded into r1 BEFORE the member at +0x14 is loaded into r0, and the
 * store to o->vptr is deferred until after both loads. Own literal pool
 * word sits at file offset 0x75c, right before the next function at
 * 0x760 — real size is 0x38, not the naive 0x34.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialDownload 0x728 0x38 xy-decomp/armcc-matching/matched/DllTrialDownload/0x728.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free218(void *p);
extern X *finish218(X *o);   /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free218(p);
        o->buf = 0;
    }
    return finish218(o);
}
