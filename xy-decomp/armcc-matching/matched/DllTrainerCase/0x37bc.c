/* DllTrainerCase function @ CRO file offset 0x37bc, size 0x8 — MATCHED
 * (masked, 2/2). Seventh match in this module.
 * Trivial virtual-override stub: `mov r0,#1; bx lr`, ignores `this`.
 * Fourth of the four `return 1` stubs in the 0x37a4-0x37e0 leaf-stub run
 * (0x37a4/0x37ac/0x37b4/0x37bc), documented as a leftover easy lead in
 * the module's README entry.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x37bc 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x37bc.c
 */

int stub(void *this)
{
    (void)this;
    return 1;
}
