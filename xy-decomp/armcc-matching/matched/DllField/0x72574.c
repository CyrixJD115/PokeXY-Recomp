/* DllField function @ CRO file offset 0x72574, size 0x50 -- MATCHED (masked, 20/20).
 * Bounds-checked lookup into a 16-entry global table: an index >= 16 calls the assert stub
 * err(0,0,0) and is reset to 0; an empty (null) slot also asserts; returns tbl[i].
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x72574 0x50 xy-decomp/armcc-matching/matched/DllField/0x72574.c
 */
extern int tbl[16];
extern void err(int, int, int);
int fn(int i)
{
    if (i >= 16) {
        err(0, 0, 0);
        i = 0;
    }
    if (tbl[i] == 0)
        err(0, 0, 0);
    return tbl[i];
}
