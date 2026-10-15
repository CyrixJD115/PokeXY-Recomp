/* DllStaffRoll function @ CRO file offset 0x2c44, size 0x38 — MATCHED
 * (masked, 14/14). Second match in this module. Same dtor idiom already
 * proven across many Dll modules (DllFatalErrorProc@0x284,
 * DllCommendation@0xc94, DllBattleHouseConnect@0x2fc, DllGameClearSave@0xd6c,
 * DllFriendEntry@0x4c4, DllEshop@0x554, DllGameSync@0x5e0, DllLiveCup@0x764):
 * installs a new vtable, frees an owned member at +0x14 if non-null, and
 * tail-calls a second import instead of returning `this` directly.
 *
 * Non-obvious part: same "good" load ordering as the proven idiom — the
 * vtable literal is loaded into r1 BEFORE the member load into r0, with
 * the store to o->vptr deferred until after both loads. Own literal pool
 * word sits at 0x2c78 (right before the next function at 0x2c7c starts),
 * which is why the real size is 0x38, not the naive 0x34. The immediate
 * sibling function at 0x2c7c has the same shape but with the known-bad
 * member-load-before-vtable-load scheduling order — skipped on sight.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStaffRoll 0x2c44 0x38 xy-decomp/armcc-matching/matched/DllStaffRoll/0x2c44.c
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
