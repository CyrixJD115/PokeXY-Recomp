/* DllTrialDownload function @ CRO file offset 0x6fd0, size 0x8 — MATCHED
 * (masked, 2/2). Twin of 0x6fc8 right next door: same constant-argument
 * tail-call forwarder idiom (r0 = 6, tail-branch to a different import
 * veneer).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialDownload 0x6fd0 0x8 xy-decomp/armcc-matching/matched/DllTrialDownload/0x6fd0.c
 */

extern int import_b(int x);

int wrapper(void)
{
    return import_b(6);
}
