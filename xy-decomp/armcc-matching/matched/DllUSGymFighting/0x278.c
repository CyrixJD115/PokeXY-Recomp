/* DllUSGymFighting function @ CRO file offset 0x278, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Sits in the recurring tail-of-import-veneer-table leaf-stub cluster
 * (44 veneer entries 0x180-0x26f, then 0x270/0x274 bare `bx lr` stubs,
 * this one, two more identical `mov r0,#0; bx lr` stubs at 0x280/0x288
 * (unclaimed), and two more bare `bx lr` stubs at 0x290/0x294) before
 * the module's first real function (a 6-register-push dispatcher) at
 * 0x298 — same shape documented across every other Dll module mined
 * so far (default virtual-method overrides on a shared base class).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymFighting 0x278 0x8 xy-decomp/armcc-matching/matched/DllUSGymFighting/0x278.c
 */

int stub_false(void *this)
{
    return 0;
}
