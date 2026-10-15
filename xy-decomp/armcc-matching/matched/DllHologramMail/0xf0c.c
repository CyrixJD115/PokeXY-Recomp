/* DllHologramMail function @ CRO file offset 0xf0c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after the module's import-veneer table (0x180-0xf07, a long
 * run of `ldr pc,[pc,#-4]` veneers) and a bare `bx lr` no-op stub at 0xf08
 * — same recurring trivial-stub-cluster shape already documented across
 * every other Dll module mined so far (default virtual-method overrides
 * on a shared dialog-proc base class). The module's first real function
 * (a destructor-shaped helper gated on two flags at +0x94/+0x98) starts
 * right after at 0xf14.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHologramMail 0xf0c 0x8 xy-decomp/armcc-matching/matched/DllHologramMail/0xf0c.c
 */

int stub_true(void *this)
{
    return 1;
}
