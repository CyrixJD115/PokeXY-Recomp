/* DllUSGymFighting function @ CRO file offset 0xdb4, size 0x28 — MATCHED (masked, 10/10).
 * Array-indexed virtual-call predicate: looks up an owned object in an
 * inline pointer array at this+0x18 (idx-th entry), calls virtual slot 55
 * (vtbl offset 0xdc) on it with a constant 0 argument, and returns
 * whether the call result is nonzero (boolean, not a tail call -- the
 * result is normalized to 0/1 and the function has its own `push {r4,lr}`
 * / `pop {r4,pc}` frame).
 *
 * The leading `add r0,r0,r1,lsl#2` (computing this+idx*4) runs *before*
 * the `push` -- armcc defers the frame-setup push past a leaf computation
 * that doesn't need it yet, same idiom already seen generally across this
 * swarm. Index scaling by 4 plus the fixed +24 immediate on the `ldr` are
 * algebraically `(this+24) + idx*4`, i.e. a `void*[]` array starting at
 * offset 0x18, not a struct-array of some other stride.
 *
 * Sits right after another small boolean-predicate function ending at
 * 0xdb0 (same vtbl-slot-0xdc idiom, two variants, unattempted) and right
 * before a larger `push {r4,r5,r6,r7,r8,r9,lr}` dispatcher at 0xddc.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymFighting 0xdb4 0x28 xy-decomp/armcc-matching/matched/DllUSGymFighting/0xdb4.c
 */

struct Obj {
    void **vtbl;
};

typedef int (*VFunc)(struct Obj *, int);

struct S {
    unsigned char pad[24];
    struct Obj *arr[1];
};

int check(struct S *this, int idx)
{
    struct Obj *obj = this->arr[idx];
    VFunc fn = (VFunc)(obj->vtbl[55]);
    return fn(obj, 0) != 0;
}
