/* DllDendouDemo @0x798 (Hall of Fame demo Dll) — 7/7 masked match.
 *
 * The module's first real function, right after the import-veneer table
 * (0x180-0x797). Calls an import veneer with `this` unchanged, and on a
 * nonzero result returns a field at +32; otherwise returns the literal 2.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllDendouDemo 0x798 0x1c \
 *       xy-decomp/armcc-matching/matched/DllDendouDemo/0x798.c
 */

typedef int s32;
typedef unsigned char u8;

extern s32 fn180(void *o);

s32 fn798(void *o)
{
    if (fn180(o) != 0) {
        return *(s32 *)((u8 *)o + 32);
    }
    return 2;
}
