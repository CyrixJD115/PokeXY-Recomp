/* DllPCMenu @0x6b8 — PC-menu Dll conditional accessor, 7/7 masked match.
 *
 * Same idiom already proven in DllLangSelect@0x5b0 / DllDendouDemo@0x798 /
 * DllBoxSearch@0x5b8 / DllDendouPC@0x650 / DllEggDemo@0x568: call an import
 * veneer with `this` unchanged; if the result is nonzero, return a member
 * field (here at +32), else re-materialize the literal `2`.
 *
 * Sits right after the module's first real function (0x550-0x6b0) and its
 * own one-word literal pool at 0x6b4 (visible in objdump as a bogus
 * `andeq r0, sl, lr, lsl r0` decode of data, not a real instruction).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllPCMenu 0x6b8 0x1c \
 *       xy-decomp/armcc-matching/matched/DllPCMenu/0x6b8.c
 */

typedef int s32;
typedef unsigned char u8;

extern s32 fn1b8(void *o);

s32 fn6b8(void *o)
{
    if (fn1b8(o) != 0) {
        return *(s32 *)((u8 *)o + 32);
    }
    return 2;
}
