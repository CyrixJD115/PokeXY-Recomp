/* DllUSBigFourFire function @ CRO file offset 0x31c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Part of the recurring tail-of-import-veneer-table leaf-stub cluster
 * already documented across every other Dll module mined so far: the
 * import veneer table runs 0x180-0x317, then a lone `bx lr` stub at
 * 0x318, three `mov r0,#0; bx lr` stubs at 0x31c/0x324/0x32c (only
 * 0x31c claimed here), another lone `bx lr` at 0x334, then the module's
 * first real function (a notification dispatcher) starts at 0x338.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSBigFourFire 0x31c 0x8 xy-decomp/armcc-matching/matched/DllUSBigFourFire/0x31c.c
 */

int stub_false(void *this)
{
    return 0;
}
