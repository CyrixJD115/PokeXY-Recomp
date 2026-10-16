/* DllTrialDownload function @ CRO file offset 0x700, size 0x8 — MATCHED
 * (masked, 2/2). Trivial virtual-override stub, `return 1`, ignores
 * `this`. Sits right after the module's import-veneer table (0x180-
 * 0x6ff); a twin stub follows at 0x710 and several bare `bx lr` single-
 * word stubs fill the rest of the tail-of-veneer-table cluster
 * (0x708/0x70c/0x714/0x718/0x71c/0x720/0x724, untried).
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialDownload 0x700 0x8 xy-decomp/armcc-matching/matched/DllTrialDownload/0x700.c
 */

typedef struct X { void *vptr; } X;

int stub(X *o)
{
    (void)o;
    return 1;
}
