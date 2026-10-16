/* DllUSBigFourSteel@0x2f4 (size 0x18) -- array-indexed virtual dispatch.
 * Looks up an owned object pointer in an inline array of pointers starting
 * at `this+0x24`, then tail-calls virtual slot 7 on that object, passing the
 * object itself as the new `this` and the caller's original third argument
 * straight through (moved r2 -> r1 before the tail bx).
 *
 * Match: 6/6 masked words (one leaf-stub `bx lr` precedes it at 0x2d4-0x2f0,
 * a separate lone `bx lr` stub follows at 0x30c -- not part of this function).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSBigFourSteel 0x2f4 0x18 \
 *       xy-decomp/armcc-matching/matched/DllUSBigFourSteel/0x2f4.c
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
