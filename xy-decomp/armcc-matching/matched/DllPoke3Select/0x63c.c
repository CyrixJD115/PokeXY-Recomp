/* DllPoke3Select function @ CRO file offset 0x63c, size 0x28 — MATCHED
 * Style: trick(redundant-local) -- named literal/temp local only to steer armcc's constant-register choice
 * (masked, 10/10). First matched function in this (sixty-sixth) module.
 *
 * Constructor: zeroes fields +0x28/+0x34/+0x38, sets a byte flag at +0x30
 * to 1, and installs a vtable pointer at +0x00 — no calls, no branches
 * (other than the return), right after the module's import-veneer table
 * (0x180-0x637) and before a lone `bx lr` weak-evidence stub at 0x664.
 *
 * Non-obvious part: register role assignment, not just value/offset
 * correctness. The retail binary keeps the constant 0 live in r1 across
 * three stores (+0x28, +0x34, +0x38) and the constant 1 live in r2 for
 * exactly one store (+0x30), then reuses r2 (now dead) to hold the
 * vtable-literal load for the +0x00 store. Writing the two constants as
 * directly-initialized plain ints in C (`zero = 0; one = 1;`) *before*
 * any field writes reproduces this exact r1/r2 role split; writing the
 * constants as literal immediates inline at each store site, or
 * introducing `vt` as the only named local (no explicit `zero`/`one`
 * locals), both made armcc swap which physical register got which role
 * (still the right shape and word count, but r1/r2 swapped throughout —
 * a 2/10 match, not 10/10). C89 block-scope rule: all three locals must
 * be declared before any statement in the function body.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPoke3Select 0x63c 0x28 xy-decomp/armcc-matching/matched/DllPoke3Select/0x63c.c
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
    int zero;
    int one;
    void *vt;
    zero = 0;
    one = 1;
    o->f28 = zero;
    o->f30 = one;
    vt = VT;
    o->f34 = zero;
    o->vptr = vt;
    o->f38 = zero;
}
