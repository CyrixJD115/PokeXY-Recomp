/* DllTrainerCase function @ CRO file offset 0x37ac, size 0x8 — MATCHED
 * (masked, 2/2). Fourth match in this module.
 * Trivial virtual-override stub: `mov r0,#1; bx lr`, ignores `this`.
 * Identical twin of 0x37a4 in the same stub cluster (0x37a4-0x37e0).
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x37ac 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x37ac.c
 */

int stub(void *this)
{
    (void)this;
    return 1;
}
