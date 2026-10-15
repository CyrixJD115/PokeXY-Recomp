/* DllField function @ CRO file offset 0x10f10, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits right after a destructor-stage function ending at 0x10f08 (plus its
 * own literal-pool word at 0x10f0c); the next function (a setter sharing
 * the same shape, see 0x10f18.c in this module) starts immediately after
 * at 0x10f18. Same recurring trivial-stub idiom documented across every
 * other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x10f10 0x8 xy-decomp/armcc-matching/matched/DllField/0x10f10.c
 */

int stub_true(void *this)
{
    return 1;
}
