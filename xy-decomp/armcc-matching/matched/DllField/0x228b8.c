/* DllField function @ CRO file offset 0x228b8, size 0x8 — MATCHED (masked, 2/2).
 * Trivial pointer-field setter: stores the second argument into
 * `this+224`, void return.
 *
 * Sits mid-module, right after a `push {r4,r5,r6,r7,r8,pc}`-returning
 * function ending at 0x228b0 (plus its own literal-pool word at 0x228b4);
 * the next function (a `push {r4,r5,lr}` constructor-shaped routine)
 * starts immediately after at 0x228c0.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x228b8 0x8 xy-decomp/armcc-matching/matched/DllField/0x228b8.c
 */

void stub_setter(void *this, void *v)
{
    *(void **)((char *)this + 224) = v;
}
