/*
 * DllStatus @ 0xa4e0 (size 0xc) -- masked match.
 * Byte-field setter taking its object through the 3rd argument register
 * (r2): sets a flag byte at +0xc to 1, unconditionally returns 1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xa4e0 0xc xy-decomp/armcc-matching/matched/DllStatus/0xa4e0.c
 */
typedef struct Obj Obj;

int fn(int a, int b, Obj *obj)
{
    (void)a;
    (void)b;
    *((unsigned char *)obj + 0xc) = 1;
    return 1;
}
