/* DllBag function @ CRO file offset 0xc80, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right at the tail of the import-veneer table (0x180-0xc7f): a run
 * of six near-identical 1-2 word leaf stubs follows immediately
 * (0xc80/0xc88/0xc90 = `return 1`, 0xc98/0xca0/0xca4/0xca8 = bare `bx lr`
 * weak-evidence stubs), then the module's first real function (a
 * 9-register-push constructor-shaped dispatcher) starts at 0xcac. Same
 * recurring tail-of-import-veneer-table leaf-stub cluster shape
 * documented for every other Dll module mined so far (DllBox@0x9e8,
 * DllFieldEventSlopeSlip@0x268, ...).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0xc80 0x8 xy-decomp/armcc-matching/matched/DllBag/0xc80.c
 */

int stub_true(void *this)
{
    return 1;
}
