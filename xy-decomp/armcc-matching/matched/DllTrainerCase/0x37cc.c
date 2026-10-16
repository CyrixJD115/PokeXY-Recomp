/* DllTrainerCase function @ CRO file offset 0x37cc, size 0x8 — MATCHED
 * (masked, 2/2). Eighth match in this module.
 * Trivial virtual-override stub: `mov r0,#1; bx lr`, ignores `this`.
 * The fifth `return 1` stub in the 0x37a4-0x37e0 leaf-stub run (after
 * 0x37a4/0x37ac/0x37b4/0x37bc, with bare `bx lr` weak-evidence stubs
 * interleaved at 0x37c4/0x37c8), right before the module's first
 * proven destructor at 0x37e4.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x37cc 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x37cc.c
 */

int stub(void *this)
{
    (void)this;
    return 1;
}
