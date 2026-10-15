/* DllTrainerCase function @ CRO file offset 0x37e4, size 0x38 — MATCHED
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * (masked, 14/14). First match in this module.
 * Standard vptr-install/free-member destructor idiom: zeroes the
 * `this+0x14` member if non-null (calling an import to free it), installs
 * a new vtable, and tail-calls a second import instead of returning via
 * pop{r4,pc} — identical shape to DllFatalErrorProc@0x284,
 * DllCommendation@0xc94, DllBattleHouseConnect@0x2fc, DllGameClearSave@0xd6c,
 * DllFriendEntry@0x4c4, DllEshop@0x554, DllGameSync@0x5e0, DllLiveCup@0x764.
 *
 * Non-obvious part: same register/instruction order rule as those
 * siblings. The retail binary loads the vtable literal into r1 FIRST,
 * then o->buf into r0, and only then stores r1 into o->vptr (deferred
 * past both loads) — the "good" ordering. Writing `o->vptr = vt;`
 * directly after `p = o->buf;` with `vt` held in its own local (computed
 * before the member read) reproduces this exact load/load/store
 * scheduling. The function's own one-word literal pool sits at 0x3818,
 * right before the next function's push{r4,lr} at 0x381c — real size is
 * 0x38, not the naive 0x34 guess (same gotcha documented for every
 * sibling in this idiom). The immediately-following sibling at 0x381c
 * has the same body shape but the known-bad member-load-before-vtable-
 * load scheduling order and was skipped on sight.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x37e4 0x38 xy-decomp/armcc-matching/matched/DllTrainerCase/0x37e4.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free208(void *p);
extern X *finish1a0(X *o);   /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free208(p);
        o->buf = 0;
    }
    return finish1a0(o);
}
