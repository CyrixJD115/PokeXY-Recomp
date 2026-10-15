/* DllGameSync function @ CRO file offset 0x5e0, size 0x38 — MATCHED (masked, 14/14).
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * First matched function in this module. Same recurring dtor idiom as
 * DllFatalErrorProc@0x284 / DllCommendation@0xc94 / DllBattleHouseConnect@0x2fc /
 * DllGameClearSave@0xd6c / DllFriendEntry@0x4c4 / DllEshop@0x554: installs a
 * new vtable, frees an owned member at +0x14 if non-null, then tail-calls a
 * second import instead of returning `this` directly (pop{r4,lr}; b <import>
 * — not pop{r4,pc}).
 *
 * Load/store scheduling matters (the already-documented residue class):
 * the retail binary loads the vtable literal into r1 FIRST, then o->buf
 * into r0, and only then stores r1 into o->vptr (deferred past both
 * loads). Writing `vt` as its own local computed before the member read,
 * then assigning `o->vptr = vt;` right after, reproduces this exact
 * scheduling. The sibling function immediately following at 0x618 has the
 * same shape but the opposite (member-load-first) order and was skipped
 * on sight as the known-bad case.
 *
 * `finish5e0` stands in for whatever the tail-called import really is
 * (masked by the verifier — a `b`, not `bl`, since lr was already popped
 * before the branch).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGameSync 0x5e0 0x38 xy-decomp/armcc-matching/matched/DllGameSync/0x5e0.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free5e0(void *p);
extern X *finish5e0(X *o);   /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free5e0(p);
        o->buf = 0;
    }
    return finish5e0(o);
}
