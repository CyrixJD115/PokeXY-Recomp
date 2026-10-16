/* DllUSBadgeCheckGate function @ CRO file offset 0x228, size 0x8 — MATCHED
 * (masked, 2/2). Trivial virtual-override stub: returns 0 (false), ignores
 * `this`. Same recurring shape documented for DllCommWait@0xcd8/
 * DllSalon@0xdc8/DllConfig@0x3a74 — sits right after the import-veneer
 * table (0x180-0x218) and a run of four bare `bx lr` single-word stubs
 * (0x218/0x21c/0x220/0x224), with two more identical return-0 stubs
 * immediately following at 0x230 and 0x238.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSBadgeCheckGate 0x228 0x8 xy-decomp/armcc-matching/matched/DllUSBadgeCheckGate/0x228.c
 */

int stub_false(void *this)
{
    return 0;
}
