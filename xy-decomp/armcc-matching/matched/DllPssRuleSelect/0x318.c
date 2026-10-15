/* DllPssRuleSelect @0x318, size 0x8 — MASKED MATCH 2/2.
 *
 * Trivial virtual-override stub: `return 1`, ignores `this`. Sits right
 * after the module's import-veneer table (0x180-0x317, 51 entries): an
 * identical twin stub follows immediately at 0x320, then two bare `bx lr`
 * single-word stubs at 0x328/0x32c (weak-evidence class, unclaimed), then
 * the module's first real function (a conditional field-reset leaf) at
 * 0x330.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllPssRuleSelect 0x318 0x8 \
 *       xy-decomp/armcc-matching/matched/DllPssRuleSelect/0x318.c
 */
int fn(void *this)
{
    return 1;
}
