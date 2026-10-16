/* DllUSGymEsper@0xad4 (size 0x18) -- array-indexed virtual dispatch.
 * Looks up an owned object pointer in an inline array of pointers starting
 * at `this+0x24`, then tail-calls virtual slot 7 on that object, passing the
 * object itself as the new `this` and the caller's original third argument
 * straight through (moved r2 -> r1 before the tail bx).
 *
 * Identical idiom/shape to DllUSBigFourSteel@0x2f4 (same member offset,
 * same vtable slot) -- ported verbatim, confirming the pattern recurs
 * across unrelated Dll modules.
 *
 * Match: 6/6 masked words. Prologue-less: a tail virtual-dispatch function
 * right after a prior function's `pop {r4,r5,r6,r7,r8,pc}` epilogue at
 * 0xad0, and a separate lone `bx lr` stub follows at 0xaec (not part of
 * this function).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSGymEsper 0xad4 0x18 \
 *       xy-decomp/armcc-matching/matched/DllUSGymEsper/0xad4.c
 */

struct Obj;

typedef long (*VFunc)(struct Obj *self, long arg);

struct Obj {
    VFunc *vtbl;
};

struct This {
    unsigned char pad[0x24];
    struct Obj *arr[1];
};

long Func(struct This *this, long idx, long arg)
{
    struct Obj *o = this->arr[idx];
    return o->vtbl[7](o, arg);
}
