/* DllPoke3Select function @ CRO file offset 0x63c, size 0x28 — MATCHED
 * (masked, 10/10). First matched function in this (sixty-sixth) module.
 * Style: natural
 * Constructor: zeroes fields +0x28/+0x34/+0x38, sets a byte flag at +0x30
 * to 1, and installs a vtable pointer at +0x00 — no calls, no branches
 * (other than the return), right after the module's import-veneer table
 * (0x180-0x637) and before a lone `bx lr` weak-evidence stub at 0x664.
 * Store order (f28, f30, f34, vptr, f38) matters for the r1/r2 register
 * roles. (Audit 2026-10-16: the earlier named `zero`/`one`/`vt` locals were
 * unnecessary -- plain literal stores in this order give the same bytes.)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPoke3Select 0x63c 0x28 xy-decomp/armcc-matching/matched/DllPoke3Select/0x63c.c
 */

typedef struct X {
    void *vptr;                 /* +0x00 */
    unsigned char pad1[0x24];   /* +0x04 */
    int f28;                    /* +0x28 */
    unsigned char pad2[0x4];    /* +0x2c */
    unsigned char f30;          /* +0x30 */
    unsigned char pad3[0x3];    /* +0x31 */
    int f34;                    /* +0x34 */
    int f38;                    /* +0x38 */
} X;

extern void *VT[];              /* vtable, pool literal */

void ctor(X *o)
{
    o->f28 = 0;
    o->f30 = 1;
    o->f34 = 0;
    o->vptr = VT;
    o->f38 = 0;
}
