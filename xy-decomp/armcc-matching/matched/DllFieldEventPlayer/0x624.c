/* DllFieldEventPlayer function @ CRO file offset 0x624, size 0x8 — MATCHED
 * (masked, 2/2), first function mined from this module. Trivial field
 * setter: stores the raw int argument into `this+32`, void return. Sits
 * right after a tail-call-forwarder's `b 0x1b8` (end of the previous
 * function) and right before a `mov r0,#1; bx lr` return-1 stub at 0x62c —
 * same tail-of-import-veneer-table leaf-stub cluster shape documented for
 * every other mined Dll module.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllFieldEventPlayer 0x624 0x8 xy-decomp/armcc-matching/matched/DllFieldEventPlayer/0x624.c
 */

void set_field(void *this, int v)
{
    *(int *)((char *)this + 32) = v;
}
