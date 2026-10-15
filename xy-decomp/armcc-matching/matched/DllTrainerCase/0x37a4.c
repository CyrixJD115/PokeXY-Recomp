/* DllTrainerCase function @ CRO file offset 0x37a4, size 0x8 — MATCHED
 * (masked, 2/2). Second match in this module.
 * Trivial virtual-override stub: `mov r0,#1; bx lr`, ignores `this`.
 * One of a run of near-identical 1-2 word leaf stubs (0x37a4-0x37e0:
 * four `return 1` stubs at 0x37a4/0x37ac/0x37b4/0x37bc, six bare `bx lr`
 * weak-evidence stubs interleaved, and a `return 1` at 0x37cc) right
 * before the module's first proven destructor at 0x37e4. Same recurring
 * shape documented in every other mined Dll module.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x37a4 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x37a4.c
 */

int stub(void *this)
{
    (void)this;
    return 1;
}
