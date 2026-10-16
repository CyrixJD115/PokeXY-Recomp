/* DllLiveCup function @ CRO file offset 0x764, size 0x38 — MASKED MATCH (14/14).
 * Style: trick(ordering-local) -- `vt = VT` local only to load the vtable literal before the member load (plain forms miss 3-4 words)
 * First matched function in this module. Same recurring dtor idiom already
 * proven in DllFatalErrorProc@0x284 / DllCommendation@0xc94 /
 * DllBattleHouseConnect@0x2fc / DllGameClearSave@0xd6c / DllFriendEntry@0x4c4 /
 * DllEshop@0x554 / DllGameSync@0x5e0: installs a new vtable, frees an owned
 * member at +0x14 if non-null, then tail-calls a second import instead of
 * returning `this` directly (pop {r4,lr}; b <import> — not pop {r4,pc}).
 *
 * Load/store scheduling matters (the already-documented residue class): the
 * retail binary loads the vtable literal into r1 FIRST, then o->buf into r0,
 * and only then stores r1 into o->vptr (deferred past both loads). Writing
 * `vt` as its own local computed before the member read, then assigning
 * `o->vptr = vt;` right after, reproduces this exact scheduling. The sibling
 * function immediately following at 0x79c has the same shape but the
 * opposite (member-load-first) order — recognized on sight as the known-bad
 * case and skipped.
 *
 * Real size is 0x38, not the naive 0x34: the function's own one-word literal
 * pool sits at file offset 0x798, right before the next function's
 * `push {r4,lr}` at 0x79c.
 *
 * `finish764` stands in for whatever the tail-called import really is
 * (masked by the verifier — a `b`, not `bl`, since lr was already popped
 * before the branch).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllLiveCup 0x764 0x38 xy-decomp/armcc-matching/matched/DllLiveCup/0x764.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free764(void *p);
extern X *finish764(X *o);   /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free764(p);
        o->buf = 0;
    }
    return finish764(o);
}
