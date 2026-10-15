/* DllFatalErrorProc function @ CRO file offset 0x284, size 0x38 — MATCHED (masked, 14/14). Second match in this module, the "0x284 easy lead" flagged alongside 0xcdc/0x2bc: same vptr-install/free-member dtor idiom, but it tail-calls a second import (b, no bl) instead of returning via pop{r4,pc} — a destructor whose caller already takes care of returning `this`-equivalent via the tail-called function.
 *
 * Non-obvious part: register/instruction order. The retail binary loads
 * the vtable literal into r1 FIRST, then o->buf into r0, and only then
 * stores r1 into o->vptr (deferred past both loads) — the opposite load
 * order from the sibling 0x2bc (which loads the member first). Writing
 * `o->vptr = vt;` directly after `p = o->buf;` with `vt` held in its own
 * local (computed before the member read) reproduces this exact
 * load/load/store scheduling; collapsing it to `o->vptr = VT;` inline
 * made armcc store eagerly and reuse one register, which lost the match.
 * `finish218` stands in for whatever is really at the tail-called import
 * (masked by the verifier; a `b`, not `bl`, since this is a true tail
 * call — lr was already popped before the branch).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllFatalErrorProc 0x284 0x38 xy-decomp/armcc-matching/matched/DllFatalErrorProc/0x284.c
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
