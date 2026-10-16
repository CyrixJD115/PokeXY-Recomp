/*
 * Style: natural
 * DllStatus @ 0xbe0 (size 0x4) -- masked match 1/1.
 * Trivial virtual-override no-op stub (lone bx lr; weak evidence).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xbe0 0x4 xy-decomp/armcc-matching/matched/DllStatus/0xbe0.c
 */
typedef struct Obj Obj;

void fn(Obj *this)
{
    (void)this;
}
