/* DllUSGymIce@0x1974 - 11/11 match
 * Init: writes -1 to one 64-bit global and 0 to another.
 * Note: the first global must be `volatile long long` to get the retail
 * `strd r0,[r2]` (plain/__align(8)/unsigned long long all give stm);
 * the second is a plain long long (stm {r1,r2}).
 * Verify: python3 tools/fingerprint_fn.py DllUSGymIce 0x180 0x2c matched/DllUSGymIce/0x180.c
 */
extern volatile long long g_a;
extern long long g_b;
void Init(void) {
    g_a = -1;
    g_b = 0;
}
