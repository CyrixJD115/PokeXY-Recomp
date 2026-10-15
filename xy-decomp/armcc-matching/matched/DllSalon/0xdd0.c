/* DllSalon function @ CRO file offset 0xdd0, size 0x2c — MATCHED (masked, 11/11).
 * Style: trick(volatile-strd) -- `volatile` on a 64-bit global only to make armcc emit `strd`
 * Globals init: writes -1 to a 64-bit slot at (array base + 8), zeroes a second 64-bit global.
 * Same family as DllUSGymFighting@0x11dc.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllSalon 0xdd0 0x2c xy-decomp/armcc-matching/matched/DllSalon/0xdd0.c
 */
extern volatile long long g_a[2];
extern long long g_b;
void Init(void) {
    g_a[1] = -1;
    g_b = 0;
}
