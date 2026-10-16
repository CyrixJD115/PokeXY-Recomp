/*
 * Style: natural
 * DllStatus @ 0xbc8 (size 0x8) -- masked match 2/2.
 * Trivial virtual-override stub: unconditionally returns 1, ignores `this`.
 * Same recurring idiom seen across nearly every Dll module in this swarm
 * (see xy-decomp/armcc-matching/README.md's "What reliably works" notes).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xbc8 0x8 xy-decomp/armcc-matching/matched/DllStatus/0xbc8.c
 */
typedef struct Obj Obj;

int fn(Obj *this)
{
    (void)this;
    return 1;
}
