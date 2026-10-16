/* DllUSDynamo function @ CRO file offset 0x1b4, size 0x8 — MATCHED (masked, 2/2). Trivial
 * virtual-override stub: returns 0 (false), ignores `this`. Same recurring idiom seen in
 * DllCommWait@0xcd8, DllConfig@0x3a74, DllSalon@0xdc8, etc.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSDynamo 0x1b4 0x8 xy-decomp/armcc-matching/matched/DllUSDynamo/0x1b4.c
 */

int stub_false(void *this)
{
    return 0;
}
