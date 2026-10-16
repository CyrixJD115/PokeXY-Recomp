/* DllPromotion function @ CRO file offset 0xc78, size 0x8 — MATCHED (masked, 2/2).
 * Style: natural
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right at the tail of the import-veneer table (0x180-0xc27): a
 * small constructor-shaped function at 0xc28 and a float-field-init
 * leaf at 0xc54 precede it, each ending in its own one-word literal
 * pool (0xc50, 0xc70-0xc74). This stub occupies exactly 0xc78-0xc7f,
 * immediately followed by a run of six bare `bx lr` single-word
 * no-op stubs (0xc80/0xc84/0xc88/0xc8c/0xc90, weak-evidence class,
 * unclaimed) and then the module's first real function (a
 * 4-register-push dispatcher) at 0xc94. Same recurring
 * tail-of-import-veneer-table leaf-stub cluster shape documented for
 * every other Dll module mined so far (DllBag@0xc80, DllBox@0x9e8,
 * DllFieldEventSlopeSlip@0x268, ...) — first candidate mined from a
 * thirty-ninth-ish module, DllPromotion.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPromotion 0xc78 0x8 xy-decomp/armcc-matching/matched/DllPromotion/0xc78.c
 */

int stub_true(void *this)
{
    return 1;
}
