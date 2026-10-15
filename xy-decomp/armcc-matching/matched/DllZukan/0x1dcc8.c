/* DllZukan function @ CRO file offset 0x1dcc8, size 0x8 -- MATCHED (masked, 2/2).
 * Sub-object address getter: returns this+4 (add r0,r0,#4; bx lr).
 * Same shape as 0x1dcd0 (+0x1c), 0x1dcd8 (+8), 0x1dce0 (+4), 0x1dce8 (+0xc).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x1dcc8 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1dcc8.c
 */

char *addr_4(char *p)
{
    return p + 4;
}
