/* DllTestKosaka@0x180 - globals init; first 64-bit store goes to base+8.
 * Verify: python3 tools/fingerprint_fn.py DllTestKosaka 0x180 0x2c matched/DllTestKosaka/0x180.c
 */
extern volatile long long g_a[2];
extern long long g_b;
void Init(void) {
    g_a[1] = -1;
    g_b = 0;
}
