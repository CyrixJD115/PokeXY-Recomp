/* DllUSPassage function @ CRO file offset 0x1620, size 0x8 — MATCHED (masked, 2/2). Trivial
 * field getter: returns the signed byte field at `this+0x5c` (sign-extended to int), no
 * branches/calls. Sits right after two bare `bx lr` no-op stubs (0x1614/0x1618/0x161c run)
 * and right before a real `push {r4,r5,r6,lr}` function at 0x1628.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPassage 0x1620 0x8 xy-decomp/armcc-matching/matched/DllUSPassage/0x1620.c
 */

int signed_byte_getter(void *this)
{
    return *(signed char *)((char *)this + 0x5c);
}
