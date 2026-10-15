/*
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * DllGts @ 0x5bc4, size 0x1c, masked match 7/7.
 *
 * Identical body to the sibling function at `0x5ba8` (same import-veneer
 * call shape, same field offsets) — only the final return constant
 * differs: 0x26 (38) here vs 0x1b (27) there. See `0x5ba8.c` for the
 * full write-up of the load-latency-hiding / writeback-merge scheduling
 * trap this pattern hits.
 *
 * Decompiled C (Ghidra, `FUN_00105bc4`):
 *   FUN_001004f4(**(undefined4 **)(param_1 + 0x10),
 *                *(undefined4 *)(param_1 + 0x30));
 *   return 0x26;
 * — see `.Data/Source_Extracted/decomp/cro_c/DllGts/00105bc4_FUN_00105bc4.c`.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x5bc4 0x1c \
 *       xy-decomp/armcc-matching/matched/DllGts/0x5bc4.c
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
    return 0x26;
}
