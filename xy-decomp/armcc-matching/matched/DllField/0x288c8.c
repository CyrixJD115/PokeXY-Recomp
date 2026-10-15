/* DllField function @ CRO file offset 0x288c8, size 0xc — MATCHED (masked, 3/3).
 * Hardcoded-zero pointer-field setter: ignores the 2nd argument entirely
 * (materializes a fresh `0` into the same register instead) and stores 0
 * at `this+0`, void return — the sibling of the plain setter at 0x288c0
 * (same field, same module), which stores the real argument instead.
 *
 * Immediately follows 0x288c0 in the same leaf-stub cluster; the next
 * instruction after this function (0x288d4) is a lone `bx lr` weak-
 * evidence stub, unclaimed.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x288c8 0xc xy-decomp/armcc-matching/matched/DllField/0x288c8.c
 */

void stub_setter(void *this, void *v)
{
    (void)v;
    *(void **)this = 0;
}
