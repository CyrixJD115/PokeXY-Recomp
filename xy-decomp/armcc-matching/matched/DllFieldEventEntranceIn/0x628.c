/* DllFieldEventEntranceIn function @ CRO file offset 0x628, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after a literal-pool word (0x624, the end of the preceding
 * real function's constant) and is immediately followed by two bare
 * `bx lr` single-word stubs (0x630/0x634, weak-evidence class) and then
 * a boolean getter (`push {r4,lr}; ldr r0,[r0,#24]; ...`) at 0x638. Same
 * recurring trivial-stub cluster shape documented for every other Dll
 * module mined so far (DllBox@0x9e8, DllBag@0xc80, etc.) — this time
 * mid-module rather than right after the import-veneer table (which
 * runs 0x180-0x2df, 44 entries, before the module's first real function
 * at 0x2e0).
 *
 * Style: natural -- trivial stub/setter (weak evidence: any function of this shape compiles identically)
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllFieldEventEntranceIn 0x628 0x8 xy-decomp/armcc-matching/matched/DllFieldEventEntranceIn/0x628.c
 */

int stub_true(void *this)
{
    return 1;
}
