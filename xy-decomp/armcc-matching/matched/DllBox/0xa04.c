/* DllBox function @ CRO file offset 0xa04, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Style: natural
 *
 * A fourth `return 1` stub, interleaved among the bare `bx lr` stubs
 * (0xa00/0xa08/0xa0c/0xa10) right before the module's first real function
 * (a 10-register-push dispatcher) at 0xa14.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0xa04 0x8 xy-decomp/armcc-matching/matched/DllBox/0xa04.c
 */

int stub_true(void *this)
{
    return 1;
}
