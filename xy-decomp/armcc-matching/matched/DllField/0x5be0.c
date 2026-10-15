/* DllField function @ CRO file offset 0x5be0, size 0x10 -- MATCHED (masked, 4/4).
 * returns (arg3 == 0): `if (c) return 0; return 1;` ignoring this and arg2.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x5be0 0x10 xy-decomp/armcc-matching/matched/DllField/0x5be0.c
 */

int is_arg3_zero(void *this, int b, int c)
{
    if (c)
        return 0;
    return 1;
}
