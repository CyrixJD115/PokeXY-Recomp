/* DllRandomTrade function @ CRO file offset 0x8b4, size 0x38 — MATCHED (masked, 14/14).
 * First matched function in this module (a seventy-seventh module for the
 * swarm). Standard destructor idiom already proven in DllFatalErrorProc@0x284/
 * DllCommendation@0xc94/DllBattleHouseConnect@0x2fc/DllGameClearSave@0xd6c/
 * DllFriendEntry@0x4c4/DllEshop@0x554/DllGameSync@0x5e0/DllLiveCup@0x764:
 * installs a new vtable pointer, frees an owned member at +0x14 if non-null,
 * tail-calls a second import instead of returning `this` directly via
 * pop{r4,pc}.
 *
 * Load/store ordering matches the "good" case from that idiom's notes: the
 * vtable literal is loaded into r1 BEFORE the member load into r0, with the
 * store to o->vptr deferred until after both loads. Real size is 0x38, not
 * the naive 0x34 — the function's own one-word literal pool sits at file
 * offset 0x8e8, right before the next function at 0x8ec.
 *
 * `finish218` stands in for the tail-called import (masked by the verifier;
 * a `b`, not `bl`, since lr was already popped before the branch).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllRandomTrade 0x8b4 0x38 xy-decomp/armcc-matching/matched/DllRandomTrade/0x8b4.c
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
