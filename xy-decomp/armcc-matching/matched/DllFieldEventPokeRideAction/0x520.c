/* DllFieldEventPokeRideAction function @ CRO file offset 0x520, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits mid-module, not at the tail of the import-veneer table: a
 * field-copy-and-store helper (`add r0,r0,#0x24; stm r0,{r1,r2}; bx lr`,
 * file offset 0x514, unattempted) ends right before it, and a real
 * `push {r4,r5,r6,lr}`-prologue function starts right after at 0x528.
 * Same recurring trivial-stub shape already documented across every
 * other Dll module mined so far (DllBox@0x9e8, DllFieldEventEntranceIn@
 * 0x628, DllFieldEventPlayer@0x624, ...).
 *
 * Style: natural -- trivial stub/setter (weak evidence: any function of this shape compiles identically)
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllFieldEventPokeRideAction 0x520 0x8 \
 *     xy-decomp/armcc-matching/matched/DllFieldEventPokeRideAction/0x520.c
 */

int stub_true(void *this)
{
    return 1;
}
