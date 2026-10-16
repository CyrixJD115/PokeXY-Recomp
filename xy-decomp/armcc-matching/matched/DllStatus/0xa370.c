/*
 * DllStatus @ 0xa370 (size 0x8) -- masked match 2/2.
 * Trivial virtual-override stub: unconditionally returns 0, ignores `this`.
 * Same recurring idiom as this module's own 0xbc8 match (there: return 1).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xa370 0x8 xy-decomp/armcc-matching/matched/DllStatus/0xa370.c
 */
typedef struct Obj Obj;

int fn(Obj *this)
{
    (void)this;
    return 0;
}
