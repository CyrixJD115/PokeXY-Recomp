/* DllSparring@0x10188 - 2/2 match
 * Trivial virtual-override stub, return 4, ignores this.
 * Sibling of the already-matched 0x10178 (return 8) and 0x10180 (return 16).
 * Verify: python3 tools/fingerprint_fn.py DllSparring 0x10188 0x8 matched/DllSparring/0x10188.c
 */
int stub(void *this) {
    (void)this;
    return 4;
}