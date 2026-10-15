/*
 * DllStatus @ 0x4590 (size 0xc) -- masked match 3/3.
 * Setter through the THIRD argument: `((u8*)arg3)[12] = 1`, no return value
 * (r0 is clobbered with the constant only as the strb source). Args 1,2 unused.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x4590 0xc xy-decomp/armcc-matching/matched/DllStatus/0x4590.c
 */
void fn(int a, int b, unsigned char *p)
{
    p[12] = 1;
}
