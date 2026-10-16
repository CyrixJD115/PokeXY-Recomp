/* DllUSGymFighting function @ CRO file offset 0x11c0, size 0x1c — MATCHED (masked, 7/7).
 * Struct zero-initializer: zeroes fields at +0, +8, +4 (in that order, not
 * sequential) plus two byte flags at +12/+13.
 *
 * Size gotcha: naive read of the 6-instruction straight-line body alone is
 * 0x18 bytes, but the function's own trailing `bx lr` is part of its body
 * -- real size is 0x1c. Discovered right after a larger `push`/`pop{...,pc}`
 * function ending with a dead literal pool at 0x11b4 (the "..." objdump
 * gap).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymFighting 0x11c0 0x1c xy-decomp/armcc-matching/matched/DllUSGymFighting/0x11c0.c
 */

struct S {
    int f0;
    int f4;
    int f8;
    unsigned char f12;
    unsigned char f13;
};

void init(struct S *o)
{
    o->f0 = 0;
    o->f8 = 0;
    o->f4 = 0;
    o->f12 = 0;
    o->f13 = 0;
}
