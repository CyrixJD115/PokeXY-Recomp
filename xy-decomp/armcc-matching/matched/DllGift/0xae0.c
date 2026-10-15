/* DllGift function @ CRO file offset 0xae0, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Right after DllGift's (unusually large, ~696-entry) import-veneer
 * table (0x180-0xadf): this stub, then a run of six bare `bx lr`
 * single-word stubs (0xae8-0xaf8, weak-evidence class), then the
 * module's first real function at 0xafc (a destructor-stage helper:
 * vtable install, free member @+0x14 if non-null, tail-call via
 * `b 0x188`) — same recurring tail-of-import-veneer-table leaf-stub
 * cluster shape documented for every other Dll module mined so far.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGift 0xae0 0x8 xy-decomp/armcc-matching/matched/DllGift/0xae0.c
 */

int stub_true(void *this)
{
    return 1;
}
