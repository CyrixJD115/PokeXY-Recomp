/* DllTradelist @ 0xb40, size 0x8 - trivial virtual-override stub, `return 1`
 * (ignores `this`). Match 2/2.
 *
 * Sits in the tail-of-import-veneer-table leaf-stub cluster (0xb40-0xc03):
 * three `return 1` stubs (0xb40/0xb50/0xb68) interleaved with six bare
 * `bx lr` single-word stubs (weak-evidence class) at 0xb48/0xb4c/0xb58/
 * 0xb5c/0xb60/0xb64 (only this one claimed), then a real constructor at
 * 0xb70.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTradelist \
 *       0xb40 0x8 xy-decomp/armcc-matching/matched/DllTradelist/0xb40.c
 */
int fn(void *this) {
    (void)this;
    return 1;
}
