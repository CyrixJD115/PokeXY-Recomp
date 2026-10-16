/* DllUSGymEsper@0x2fc (size 0x8) -- trivial virtual-override stub,
 * `return 0;`, ignores `this`.
 *
 * Match: 2/2 masked words. Sits in the recurring tail-of-import-veneer-
 * table leaf-stub cluster (import veneers 0x180-0x2f7, four lone `bx lr`
 * stubs at 0x2f8/0x30c/0x320/0x324, a twin `return 0` stub at 0x304
 * unclaimed) seen in every Dll module mined so far.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSGymEsper 0x2fc 0x8 \
 *       xy-decomp/armcc-matching/matched/DllUSGymEsper/0x2fc.c
 */

int F(void *this)
{
    (void)this;
    return 0;
}
