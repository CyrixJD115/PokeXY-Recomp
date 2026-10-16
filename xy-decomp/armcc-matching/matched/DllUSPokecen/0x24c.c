/* DllUSPokecen function @ CRO file offset 0x24c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 * Part of the recurring stub cluster right after the import-veneer table
 * (0x180-0x23f): three bare `bx lr` stubs at 0x240/0x244/0x248, then three
 * `mov r0,#0; bx lr` stubs at 0x24c/0x254/0x25c (only 0x24c claimed here),
 * another bare `bx lr` at 0x264, then the module's first real function
 * (a constructor) at 0x268.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPokecen 0x24c 0x8 xy-decomp/armcc-matching/matched/DllUSPokecen/0x24c.c
 */

int stub_false(void *this)
{
    return 0;
}
