/* DllPokeList function @ CRO file offset 0x8ec, size 0x8 -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 1 (true), ignores `this`.
 * Sibling of the 0x8dc stub in the same leaf-stub cluster.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPokeList 0x8ec 0x8 xy-decomp/armcc-matching/matched/DllPokeList/0x8ec.c
 */

int stub_true(void *this)
{
    return 1;
}
