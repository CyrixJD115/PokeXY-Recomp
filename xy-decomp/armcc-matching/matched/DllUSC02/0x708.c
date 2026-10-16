/* DllUSC02 function @ CRO file offset 0x708, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Sits right after a constructor-shaped function ending at 0x6fc (pop
 * {r4, pc}) plus its own 2-word literal pool at 0x700/0x704, and right
 * before a bare `bx lr` no-op stub at 0x710 and a one-line accessor/tail-
 * call at 0x714 (`ldr r0,[r0,#32]; b <veneer>`) — same recurring trivial-
 * stub-cluster shape already documented across every other Dll module
 * mined so far (default virtual-method overrides on a shared base class),
 * this time sitting mid-module rather than right after the import-veneer
 * table or at the tail.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSC02 0x708 0x8 xy-decomp/armcc-matching/matched/DllUSC02/0x708.c
 */

int stub_false(void *this)
{
    return 0;
}
