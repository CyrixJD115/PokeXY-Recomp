/*
 * Style: trick(ordering-local) -- locals (`t`, `f30`) hoisting the two member loads in source order only to reproduce the pre-indexed `ldr r2,[r0,#16]!` scheduling (single-expression form misses 3 words)
 * DllGts @ 0x53e4, size 0x1c (NOT 0x24 — see note), masked match 7/7.
 *
 * Third sibling of `0x5ba8`/`0x5bc4`: identical double-pointer-deref
 * call+return body (`fn4f4(**field10, field30)`), returns 0x13 (19).
 *
 * Decompiled C (Ghidra, `FUN_001053e4`) agrees and is itself only 7
 * instructions / 0x1c bytes — see
 * `.Data/Source_Extracted/decomp/cro_c/DllGts/001053e4_FUN_001053e4.c`.
 *
 * Size note: the candidate scanner flagged this as size 0x24. The real
 * function body ends at the `pop {r4,pc}` (0x53fc), 0x1c bytes in; the
 * next 8 bytes (`sub r0,r0,#8` then a `mov r0,r0` nop, ending at 0x5408)
 * are unrelated dead/padding bytes before the next real function's
 * prologue (`push {r4,r5,r6,r7,r8,r9,lr}` at 0x5408), not part of this
 * one — comparing the full 0x24 window would spuriously fail on bytes
 * this function never emits. Fingerprinted and matched at the corrected
 * size 0x1c, mirroring the Ghidra function boundary.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x53e4 0x1c \
 *       xy-decomp/armcc-matching/matched/DllGts/0x53e4.c
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
    return 0x13;
}
