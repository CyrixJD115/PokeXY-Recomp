/* DllTestKawada@0x180 - 9/9 match (pool words are link-time relocs, zero in file)
 * Module-init style stub: A (64-bit) = -1; B (64-bit) = 0.
 * Trick: A must be `volatile long long` -- plain long long emits stm/str
 * pairs, only a volatile 64-bit store gets armcc to emit `strd r0,[r2]`.
 * B stays a plain long long (zero via mov/mov/stm).
 * Same bytes (9/9) also verified at 0x180/0x24 in DllFieldDebug,
 * DllTestIchiraku, DllTestNagihashi, DllTestSuzuki. DllTestKosaka differs by
 * one word only (strd r0,[r2,#8] -- A sits at +8 of a global struct).
 * Verify: python3 tools/fingerprint_fn.py DllTestKawada 0x180 0x24 matched/DllTestKawada/0x180.c
 */
extern volatile long long A;
extern long long B;
void init(void) {
    A = -1LL;
    B = 0;
}
