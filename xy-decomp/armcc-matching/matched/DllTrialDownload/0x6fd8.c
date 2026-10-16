/* DllTrialDownload function @ CRO file offset 0x6fd8, size 0x14 — MATCHED
 * (masked, 5/5). Prologue-less leaf: zeroes three fields of the argument
 * struct (two 32-bit ints then a 16-bit half) and returns void. Sits
 * right after the pair of constant-argument forwarders at 0x6fc8/0x6fd0.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialDownload 0x6fd8 0x14 xy-decomp/armcc-matching/matched/DllTrialDownload/0x6fd8.c
 */

typedef struct X {
    int a;        /* +0x0 */
    int b;        /* +0x4 */
    short c;      /* +0x8 */
} X;

void clear(X *o)
{
    o->a = 0;
    o->b = 0;
    o->c = 0;
}
