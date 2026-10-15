/*
 * DllWazaOboe @ 0x1024, size 0x4, masked match 1/1.
 * Trivial no-op void stub (single `bx lr`) — weak evidence, same caveat as
 * 0x804: any void function with no observable side effect compiles to this
 * same one word. Sits right after the thin tail-call forwarder at 0x1020,
 * right before a real function at 0x1028 (push {r4-r8,lr}).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllWazaOboe \
 *       0x1024 0x4 xy-decomp/armcc-matching/matched/DllWazaOboe/0x1024.c
 */
void fn(void *this) {
    (void)this;
}
