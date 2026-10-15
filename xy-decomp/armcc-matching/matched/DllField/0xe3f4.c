/* DllField function @ CRO file offset 0xe3f4, size 0x8 -- MATCHED (masked, 2/2).
 * trivial virtual stub: returns -1 (mvn r0,#0; bx lr).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xe3f4 0x8 xy-decomp/armcc-matching/matched/DllField/0xe3f4.c
 */

int stub_minus1(void *this)
{
    return -1;
}
