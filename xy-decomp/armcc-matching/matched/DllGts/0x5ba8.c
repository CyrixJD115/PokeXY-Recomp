/*
 * DllGts @ 0x5ba8, size 0x1c, masked match 7/7.
 *
 * Call+return helper: dereferences `this->field10` (a pointer-to-pointer
 * at offset 0x10) once to get a pointer, loads `this->field30` (offset
 * 0x30), dereferences the first pointer a second time, and forwards both
 * values to an import veneer (`fn4f4(**field10, field30)`), then returns
 * the constant 0x1b (27).
 *
 * Decompiled C (Ghidra, `FUN_00105ba8`):
 *   FUN_001004f4(**(undefined4 **)(param_1 + 0x10),
 *                *(undefined4 *)(param_1 + 0x30));
 *   return 0x1b;
 * — see `.Data/Source_Extracted/decomp/cro_c/DllGts/00105ba8_FUN_00105ba8.c`.
 *
 * Reverse-engineering note: the retail binary uses an ARM pre-indexed
 * load (`ldr r2,[r0,#16]!`) to fetch `field10`'s pointer VALUE while
 * simultaneously advancing r0 to `this+0x10`, then a plain offset load
 * off that advanced base (`ldr r1,[r0,#32]` = `this+0x10+0x20` =
 * `this+0x30` = `field30`), and only THEN performs the second
 * dereference (`ldr r0,[r2]`) to finish computing the first call
 * argument — classic load-latency hiding: start the dependent chain's
 * first load early, interleave an independent load, finish the
 * dependent chain last. Writing the whole thing as one expression
 * (`fn4f4(**(int**)(p), *(int*)(p+0x20))`, or even with a manually
 * advanced cursor pointer `p += 0x10` evaluated right before the call)
 * made armcc schedule `field30` first with NO writeback merge at all
 * (3/7 words off: plain `[r0,#16]` + `[r0,#48]` loads, no `!`). Only
 * splitting it into separate statements in this exact order — local
 * `t = this->field10` (first), local `f30 = this->field30` (second),
 * THEN `*(int*)t` inline in the call — reproduced the writeback-merge
 * scheduling exactly. Sibling: `0x5bc4` (identical body, returns 0x26).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x5ba8 0x1c \
 *       xy-decomp/armcc-matching/matched/DllGts/0x5ba8.c
 */

typedef struct Obj {
    unsigned char pad1[0x10];
    void **field10;
    unsigned char pad2[0x1c];
    void *field30;
} Obj;

extern int fn4f4(int a, int b);

int F(Obj *this)
{
    void **t = this->field10;
    void *f30 = this->field30;
    fn4f4(*(int *)t, (int)f30);
    return 0x1b;
}
