/* DllUSPassage function @ CRO file offset 0x334, size 0x8 — MATCHED (masked, 2/2). Trivial
 * virtual-override stub: returns 0 (false), ignores `this`. Same recurring idiom seen in
 * DllUSDynamo@0x1b4, DllCommWait@0xcd8, DllConfig@0x3a74, DllSalon@0xdc8, etc. Sits in a
 * run of three identical return-0 stubs right after the import-veneer table
 * (0x180-0x32f): 0x330 is a lone `bx lr`, then this stub at 0x334/0x338, another at
 * 0x33c/0x340, another at 0x344/0x348, then a final lone `bx lr` at 0x34c before the
 * module's first real function at 0x350.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPassage 0x334 0x8 xy-decomp/armcc-matching/matched/DllUSPassage/0x334.c
 */

int stub_false(void *this)
{
    return 0;
}
