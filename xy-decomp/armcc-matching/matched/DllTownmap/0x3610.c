/* DllTownmap function @ CRO file offset 0x3610, size 0x2c - init two globals (strd -1,-1 ; stm 0,0).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x3610 0x2c xy-decomp/armcc-matching/matched/DllTownmap/0x3610.c
 */
extern volatile long long g_a;
extern long long g_b;
void init_globals(void)
{
    g_a = -1;
    g_b = 0;
}
