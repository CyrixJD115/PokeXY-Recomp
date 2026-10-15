/* DllField function @ CRO file offset 0x10f18, size 0xc — MATCHED (masked, 3/3).
 * Byte-field setter + hardcoded-true return: stores the *third* argument
 * (r2, not r1 — the second argument is unused) into the byte field at
 * `this+141`, then unconditionally returns 1.
 *
 * Right after the trivial `return 1` stub at 0x10f10 (same module, same
 * cluster); the next function (a `push {r4,r5,r6,r7,lr}` dispatcher)
 * starts immediately after at 0x10f24.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x10f18 0xc xy-decomp/armcc-matching/matched/DllField/0x10f18.c
 */

int stub_setter(void *this, unsigned int a1, unsigned int a2)
{
    *((unsigned char *)this + 141) = (unsigned char)a2;
    (void)a1;
    return 1;
}
