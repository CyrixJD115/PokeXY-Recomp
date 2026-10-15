/* DllSubScreenTutorial@0x1338 (size 0x8) -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub, `return 0`, ignores `this`.
 * Sits right after the known-bad member-load-first sibling destructor
 * at 0x1304 (same idiom as DllFatalErrorProc@0x2bc, but with the
 * known-bad scheduling order per the toolchain's residue-class notes --
 * skipped on sight, not attempted) and right before a real
 * push {r4,r5,r6,lr} function at 0x1340.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSubScreenTutorial 0x1338 0x8 \
 *       xy-decomp/armcc-matching/matched/DllSubScreenTutorial/0x1338.c
 */

int G(void *this)
{
    (void)this;
    return 0;
}
