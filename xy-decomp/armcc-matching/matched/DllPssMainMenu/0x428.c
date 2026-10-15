/* DllPssMainMenu function @ CRO file offset 0x428, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * First function mined from DllPssMainMenu (the Pokemon-Global-Link /
 * PSS "main menu" Dll, a module unrelated to the Dll*Input dialog-proc
 * family). Sits in the usual tail-of-import-veneer-table leaf-stub
 * cluster: the import-veneer table runs 0x180-0x427, then two identical
 * `mov r0,#1; bx lr` stubs at 0x428/0x430 (only 0x428 claimed), then two
 * bare `bx lr` single-word stubs at 0x438/0x43c (weak-evidence class,
 * unclaimed), then the module's first real function (a dispatcher that
 * calls through several veneers) starts at 0x440.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPssMainMenu 0x428 0x8 xy-decomp/armcc-matching/matched/DllPssMainMenu/0x428.c
 */

int stub_true(void *this)
{
    return 1;
}
