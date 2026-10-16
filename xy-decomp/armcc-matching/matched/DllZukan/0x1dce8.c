/* DllZukan function @ CRO file offset 0x1dce8, size 0x8 -- MATCHED (masked, 2/2).
 * Sub-object address getter: returns this+0xc (add r0,r0,#12; bx lr).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x1dce8 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1dce8.c
 */

char *addr_c(char *p)
{
    return p + 12;
}
