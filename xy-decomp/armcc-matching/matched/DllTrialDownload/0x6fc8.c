/* DllTrialDownload function @ CRO file offset 0x6fc8, size 0x8 — MATCHED
 * (masked, 2/2). Thin tail-call forwarder with a constant argument: sets
 * r0 to the literal 6 and tail-branches straight into an import veneer
 * (no `bl`/`pop` — void-returning call whose result IS the caller's
 * result, so no stack frame is needed). A near-identical twin follows
 * immediately at 0x6fd0, forwarding the same literal 6 into a different
 * import.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialDownload 0x6fc8 0x8 xy-decomp/armcc-matching/matched/DllTrialDownload/0x6fc8.c
 */

extern int import_a(int x);

int wrapper(void)
{
    return import_a(6);
}
