/* DllTradelist @ 0xb5f0, size 0x8 - trivial virtual-override stub, `return 1`
 * (ignores `this`). Match 2/2.
 *
 * Sits right after a larger prologue'd dispatcher (push {r4,lr}...pop
 * {r4,pc}) ending at 0xb5ec, and right before a call-setup helper at
 * 0xb5f8 that sets up args and tail-calls a shared routine.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTradelist \
 *       0xb5f0 0x8 xy-decomp/armcc-matching/matched/DllTradelist/0xb5f0.c
 */
int fn(void *this) {
    (void)this;
    return 1;
}
