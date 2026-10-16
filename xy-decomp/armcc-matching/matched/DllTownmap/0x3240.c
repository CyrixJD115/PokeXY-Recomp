/* DllTownmap function @ CRO file offset 0x3240, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * A one-word literal pool (0x323c) precedes it; the double-indirection
 * getter at 0x3248 (ldr r0,[r0,#4]; ldr r0,[r0]; bx lr) follows
 * immediately. Same recurring trivial-stub idiom seen tail/mid-module
 * in every Dll module mined so far.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x3240 0x8 xy-decomp/armcc-matching/matched/DllTownmap/0x3240.c
 */

int stub_false(void *this)
{
    return 0;
}
