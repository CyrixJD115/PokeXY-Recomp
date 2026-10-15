/* DllReport @0x728 (report/record Dll) — 7/7 masked match.
 *
 * Module's first real function right after the import-veneer table
 * (0x180-0x727). Same conditional-accessor idiom already proven in
 * DllLangSelect@0x5b0 / DllDendouDemo@0x798 / DllBoxSearch@0x5b8 /
 * DllDendouPC@0x650 / DllEggDemo@0x568 / DllPCMenu@0x6b8: calls an import
 * veneer with `this` unchanged, and on a nonzero result returns a member
 * field (here at +84); otherwise returns the re-materialized literal 2.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllReport 0x728 0x1c \
 *       xy-decomp/armcc-matching/matched/DllReport/0x728.c
 */

typedef int s32;
typedef unsigned char u8;

extern s32 fn180(void *o);

s32 fn728(void *o)
{
    if (fn180(o) != 0) {
        return *(s32 *)((u8 *)o + 84);
    }
    return 2;
}
