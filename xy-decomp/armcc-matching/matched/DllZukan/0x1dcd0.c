/* DllZukan function @ CRO file offset 0x1dcd0, size 0x8 -- MATCHED (masked, 2/2).
 * Sub-object address getter: returns this+0x1c (add r0,r0,#28; bx lr).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x1dcd0 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1dcd0.c
 */

char *addr_1c(char *p)
{
    return p + 28;
}
