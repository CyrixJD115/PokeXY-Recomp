/* DllField function @ CRO file offset 0x288c0, size 0x8 — MATCHED (masked, 2/2).
 * Trivial pointer-field setter: stores the second argument at `this+0`
 * (offset 0), void return.
 *
 * Right after a notification-dispatcher-shaped function ending at 0x288bc
 * (`b <local 0x17e8>` tail then `pop {r4,r5,r6,pc}`); the very next
 * function at 0x288c8 is the sibling "hardcoded-zero" variant of this
 * same setter (see 0x288c8.c in this module) — the real argument is
 * discarded there and 0 is stored instead.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x288c0 0x8 xy-decomp/armcc-matching/matched/DllField/0x288c0.c
 */

void stub_setter(void *this, void *v)
{
    *(void **)this = v;
}
