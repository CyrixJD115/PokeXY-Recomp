/* DllSubScreenTutorial@0x954 (size 0x8) -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub, `return 0`, ignores `this`.
 * Sits right after two near-identical multi-case dispatcher functions
 * (one ending at 0x928, a second ending at 0x94c, both sharing the same
 * tail shape -- not attempted) and right before a real
 * push {r4,r5,lr} function at 0x95c.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSubScreenTutorial 0x954 0x8 \
 *       xy-decomp/armcc-matching/matched/DllSubScreenTutorial/0x954.c
 */

int G(void *this)
{
    (void)this;
    return 0;
}
