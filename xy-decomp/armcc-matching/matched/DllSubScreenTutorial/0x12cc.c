/* DllSubScreenTutorial@0x12cc (size 0x38) -- MATCHED (masked, 14/14).
 * Destructor: same idiom already proven in DllFatalErrorProc@0x284/
 * DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/
 * DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764:
 * installs a new vtable, frees an owned member at +0x14 if non-null,
 * tail-calls a second import instead of returning `this` directly.
 * Same "good" load ordering (vtable literal into r1 before the member
 * load into r0, store deferred until after both loads) and the same
 * size gotcha: the function's own one-word literal pool sits at
 * 0x1300, right before the next function at 0x1304 -- real size is
 * 0x38, not the naive 0x34.
 *
 * An almost-identical sibling destructor immediately follows at 0x1304
 * (same shape, but returns via `pop {r4,pc}` instead of tail-calling --
 * same idiom as DllFatalErrorProc@0x2bc) and was not attempted this run.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSubScreenTutorial 0x12cc 0x38 \
 *       xy-decomp/armcc-matching/matched/DllSubScreenTutorial/0x12cc.c
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
