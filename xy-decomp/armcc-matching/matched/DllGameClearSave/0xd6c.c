/* DllGameClearSave function @ CRO file offset 0xd6c, size 0x38 — MATCHED
 * (masked, 14/14). First matched function in this (eighth) module.
 *
 * Destructor: vtable-install + free-one-owned-member idiom, same shape as
 * DllFatalErrorProc@0x284 and DllDialogCommon@0xcdc (same member offset
 * +0x14) — tail-calls a second import instead of returning `this` via
 * pop{r4,pc}. Non-obvious part is instruction order, same trap documented
 * for DllFatalErrorProc@0x284: the retail binary loads the vtable literal
 * into r1 FIRST, then the member (`o->buf`) into r0, and only then stores
 * r1 into `o->vptr` (deferred past both loads). Writing `vt = VT;` as its
 * own local computed before the member read, then `o->vptr = vt;` right
 * after the member read, reproduces this exact load/load/store scheduling.
 * `finish_d68` stands in for whatever is really at the tail-called import
 * (masked by the verifier; a `b`, not `bl`, since this is a true tail call
 * — lr was already popped before the branch). Size note: the function's
 * own literal pool (one word, the vtable address) sits right at the
 * 13-instruction/14-word boundary, immediately before the next function
 * at 0xda4 — matched on the first try only once the size was corrected
 * from 0x34 (13 words, excluding the pool) to 0x38 (14 words, including
 * it); at 0x34 the candidate's own compiled pool word had nothing to
 * compare against.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGameClearSave 0xd6c 0x38 xy-decomp/armcc-matching/matched/DllGameClearSave/0xd6c.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free_d60(void *p);
extern X *finish_d68(X *o);  /* tail-called import, identity masked */

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free_d60(p);
        o->buf = 0;
    }
    return finish_d68(o);
}
