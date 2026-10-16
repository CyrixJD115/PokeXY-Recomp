/* DllTestKosaka@0x180 - globals init: writes -1 to the 64-bit global at g_a+8 and 0 to g_b.
 * Style: trick(volatile-strd) -- `volatile` on a 64-bit global only to make armcc emit `strd`
 * Verify: python3 tools/fingerprint_fn.py DllTestKosaka 0x180 0x2c matched/DllTestKosaka/0x180.c
 */
extern volatile long long g_a[2];
extern long long g_b;
void Init(void) {
    g_a[1] = -1;
    g_b = 0;
}
