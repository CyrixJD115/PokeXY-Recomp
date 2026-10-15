/* DllUSGymGrass function @ CRO file offset 0x22c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Part of the cluster of 1-2 word leaf functions right after the import
 * veneer table (0x180-0x227) and before the first real function at 0x250
 * -- same shape already documented across every other Dll module mined
 * so far (default virtual-method overrides on a shared base class). The
 * surrounding cluster has a lone `bx lr` at 0x228, three `mov r0,#0; bx
 * lr` stubs at 0x22c/0x234/0x23c (only 0x22c claimed here), then three
 * more lone `bx lr` stubs at 0x244/0x248/0x24c before the real code
 * starts at 0x250.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymGrass 0x22c 0x8 xy-decomp/armcc-matching/matched/DllUSGymGrass/0x22c.c
 */

int stub_false(void *this)
{
    return 0;
}
