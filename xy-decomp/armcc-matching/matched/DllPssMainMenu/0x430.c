/* DllPssMainMenu@0x430 - 2/2 match
 * Style: natural
 * Trivial virtual-override stub, return 1, ignores this.
 * Twin of the already-matched 0x428.
 * Verify: python3 tools/fingerprint_fn.py DllPssMainMenu 0x430 0x8 matched/DllPssMainMenu/0x430.c
 */
int stub(void *this) {
    (void)this;
    return 1;
}