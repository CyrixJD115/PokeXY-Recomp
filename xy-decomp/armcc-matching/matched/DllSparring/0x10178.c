/* DllSparring function @ CRO file offset 0x10178, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns the literal 8, ignores `this`.
 *
 * First module mined for DllSparring. `.text` starts at file offset
 * 0x180 with an unusually large import-veneer table (244 entries,
 * 0x180-0x91f), followed by the module's first real function (a large
 * constructor/initializer, ~0x8c0 bytes) at 0x920. This stub sits in a
 * run of three near-identical 2-word leaf constant-getters right after
 * a larger dispatcher function ending at 0x10174: `return 8` here,
 * `return 16` at 0x10180, `return 4` at 0x10188 (siblings unclaimed),
 * then the module's next real function (a `push {r4-sl,lr}` dispatcher)
 * starts at 0x10190. Same recurring tail-of-dispatcher trivial-stub
 * cluster shape documented across every other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSparring 0x10178 0x8 xy-decomp/armcc-matching/matched/DllSparring/0x10178.c
 */

int stub_return8(void *this)
{
    return 8;
}
