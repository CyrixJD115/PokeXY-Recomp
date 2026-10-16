/* DllStaffRoll function @ CRO file offset 0x59c, size 0x8 — MATCHED
 * (masked, 2/2). First function matched in this module. Trivial
 * virtual-override stub: unconditional `return 1;`, ignores `this`.
 * One of four near-identical `return 1` stubs at 0x59c/0x5a4/0x5ac/0x5b4
 * (only this one claimed), right after the module's import-veneer table
 * (0x180-0x598) and before five bare `bx lr` weak-evidence stubs
 * (0x5bc-0x5cc), which precede the module's first real function (a
 * 5-case jump-table dispatcher at 0x5d0 with the known-hard
 * jump-table/NOP-hint residue class — not attempted).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStaffRoll 0x59c 0x8 xy-decomp/armcc-matching/matched/DllStaffRoll/0x59c.c
 * Style: natural
 */

long func(void *this) {
    (void)this;
    return 1;
}
