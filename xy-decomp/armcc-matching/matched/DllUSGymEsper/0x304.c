/* DllUSGymEsper@0x304 (size 0x8) -- trivial virtual-override stub,
 * `return 0;`, ignores `this`. Byte-identical twin of 0x2fc, right next
 * to it in the same tail-of-import-veneer-table leaf-stub cluster.
 *
 * Match: 2/2 masked words.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSGymEsper 0x304 0x8 \
 *       xy-decomp/armcc-matching/matched/DllUSGymEsper/0x304.c
 */

int F(void *this)
{
    (void)this;
    return 0;
}
