/* DllUS3DDun function @ CRO file offset 0x24c, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Part of the cluster of 1-2 word leaf functions right after the import
 * veneer table (0x180-0x247) and before the first real function at 0x274
 * -- same shape already documented across every other Dll module mined
 * so far (default virtual-method overrides on a shared base class).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUS3DDun 0x24c 0x8 xy-decomp/armcc-matching/matched/DllUS3DDun/0x24c.c
 */

int stub_false(void *this)
{
    return 0;
}
