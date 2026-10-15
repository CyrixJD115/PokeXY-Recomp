/* DllUSPokecen@0x254 - 2/2 match
 * Trivial virtual-override stub, return 0, ignores this.
 * Sibling of the already-matched 0x24c (also return 0).
 * Verify: python3 tools/fingerprint_fn.py DllUSPokecen 0x254 0x8 matched/DllUSPokecen/0x254.c
 */
int stub(void *this) {
    (void)this;
    return 0;
}