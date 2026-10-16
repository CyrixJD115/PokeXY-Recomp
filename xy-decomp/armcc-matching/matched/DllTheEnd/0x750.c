/* DllTheEnd function @ CRO file offset 0x750, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * First function mined from DllTheEnd (the end-credits/ending-sequence Dll,
 * a module unrelated to the Dll*Input dialog-proc family). Sits isolated
 * between two unrelated helper functions in the tail-of-vtable stub
 * cluster already documented for every other mined module: a one-word
 * literal pool at 0x74c (belonging to the previous function's epilogue)
 * immediately precedes it, and the next function (ldr r0,[r0,#36]; ...)
 * starts immediately after at 0x758 — exactly the 0x8-byte isolated shape
 * seen at DllNumberInput@0x1888 and the other `return 1` stubs.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTheEnd 0x750 0x8 xy-decomp/armcc-matching/matched/DllTheEnd/0x750.c
 */

int stub_true(void *this)
{
    return 1;
}
