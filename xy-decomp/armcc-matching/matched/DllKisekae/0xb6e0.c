/* DllKisekae function @ CRO file offset 0xb6e0, size 0x8 — MASKED MATCH (2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * First function matched in this module. Sits in the classic recurring
 * tail-of-vtable leaf-stub cluster: a destructor-stage helper (vptr install,
 * conditional free of a member via a masked `bl`, return `this`) ends right
 * before it at 0xb6ac-0xb6db (own literal pool at 0xb6dc); then two
 * identical `return 1` stubs at 0xb6e0 and 0xb6e8 (only 0xb6e0 claimed
 * here); then a `mov r1,#1; strb r1,[r0,#0x40a]; bx lr` hardcoded-true
 * byte-field setter at 0xb6f0 (untried, 0xc bytes); then the next real
 * function (a 7-register-push dispatcher) starts at 0xb6fc. Same shape
 * already proven dozens of times across this swarm's mined modules
 * (DllDialogCommon@0x2d8, DllFieldEventSlopeSlip@0x268, etc.).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllKisekae 0xb6e0 0x8 xy-decomp/armcc-matching/matched/DllKisekae/0xb6e0.c
 */

int stub_true(void *this)
{
    return 1;
}
