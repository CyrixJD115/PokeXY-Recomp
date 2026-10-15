/* DllSparring@0x10180 - 2/2 match
 * Trivial virtual-override stub, return 16, ignores this.
 * Sibling of the already-matched 0x10178 (return 8).
 * Verify: python3 tools/fingerprint_fn.py DllSparring 0x10180 0x8 matched/DllSparring/0x10180.c
 */
int stub(void *this) {
    (void)this;
    return 16;
}