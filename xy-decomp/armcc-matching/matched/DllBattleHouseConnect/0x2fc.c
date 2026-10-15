/* DllBattleHouseConnect function @ CRO file offset 0x2fc, size 0x38 — MATCHED
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * (masked, 14/14). First match from an eighth module. Destructor: vptr
 * install, frees member @+0x14, tail-calls a second import instead of
 * returning `this` directly — the same idiom already proven at
 * DllFatalErrorProc@0x284 (vtable literal loaded into r1 BEFORE the member
 * load into r0, store deferred until after both loads, then
 * `pop {r4,lr}; b <import>` as a true tail call since lr is already
 * popped before the branch).
 *
 * Non-obvious part: size. The naive guess (0x34, matching the sibling
 * non-tail-call dtor shape) leaves the function's own literal pool word
 * (the `VT` pointer at file offset 0x330) unaccounted for — it sits right
 * before the next function's `push {r4, lr}` at 0x334, so the real size is
 * 0x38, one word more, exactly the `0x284` lesson repeated.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattleHouseConnect 0x2fc 0x38 xy-decomp/armcc-matching/matched/DllBattleHouseConnect/0x2fc.c
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
