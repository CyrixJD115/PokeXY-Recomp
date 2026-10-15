/* DllTrainerCase function @ CRO file offset 0x3850, size 0x8 — MATCHED
 * (masked, 2/2). Third match in this module.
 * Trivial virtual-override stub: `mov r0,#0; bx lr`, ignores `this`.
 * Sits right after a pair of the module's known-bad-ordering
 * destructor-stage helpers (0x37e4 matched, 0x381c skipped — member-
 * load-before-vtable-load ordering), same recurring tail-of-cluster
 * shape documented in every other mined Dll module.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x3850 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x3850.c
 */

int stub(void *this)
{
    (void)this;
    return 0;
}
