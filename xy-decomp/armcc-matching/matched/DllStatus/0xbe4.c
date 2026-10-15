/*
 * DllStatus @ 0xbe4 (size 0x8) -- masked match 2/2.
 * Trivial virtual-override stub: returns 1, ignores this.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xbe4 0x8 xy-decomp/armcc-matching/matched/DllStatus/0xbe4.c
 */
typedef struct Obj Obj;

int fn(Obj *this)
{
    (void)this;
    return 1;
}
