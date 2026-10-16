/* DllZukan function @ CRO file offset 0xf9c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * Sits in the recurring tail-of-import-veneer-table leaf-stub cluster
 * (0xf98-0xfd7): the veneer table itself runs 0x180-0xf97, then a bare
 * `bx lr` at 0xf98, this stub, a twin at 0xfa4, another twin at 0xfac,
 * two bare `bx lr` at 0xfb4/0xfb8, a fourth `return 1` stub at 0xfbc, and
 * five more bare `bx lr` weak-evidence stubs through 0xfd7 — same shape
 * already documented across every other Dll module mined by this swarm
 * (DllDialogCommon@0x2d8, DllFieldEventSlopeSlip@0x268, etc.). First real
 * function (a destructor/init helper) starts right after at 0xfd8.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0xf9c 0x8 xy-decomp/armcc-matching/matched/DllZukan/0xf9c.c
 */

int stub_true(void *this)
{
    return 1;
}
