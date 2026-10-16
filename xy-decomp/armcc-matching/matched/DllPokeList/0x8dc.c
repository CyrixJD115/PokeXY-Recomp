/* DllPokeList function @ CRO file offset 0x8dc, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Right after the import-veneer table (0x180-0x8db): a run of near-
 * identical 1-2 word leaf stubs at the tail of the table —
 * `return 1` stubs at 0x8dc/0x8e4/0x8ec/0x8f8 (only 0x8dc claimed
 * here) interleaved with bare `bx lr` single-word stubs (weak-evidence
 * class) at 0x8f4/0x900/0x904/0x908/0x90c — then the module's first
 * real function (a `push {r4-r8,lr}` constructor-shaped dispatcher) at
 * 0x910. Same recurring tail-of-import-veneer-table leaf-stub cluster
 * shape documented for every other Dll module mined so far (e.g.
 * DllBox@0x9e8, DllPssPlaySelect@0x2c8).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPokeList 0x8dc 0x8 xy-decomp/armcc-matching/matched/DllPokeList/0x8dc.c
 */

int stub_true(void *this)
{
    return 1;
}
