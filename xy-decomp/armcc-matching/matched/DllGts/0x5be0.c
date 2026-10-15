/*
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * DllGts @ 0x5be0, size 0x3c, masked match 15/15.
 *
 * Call+return helper: forwards `this->field28` and `this->field550` to an
 * import veneer (`fn214`, args: obj, 1, 38, field550), zeroes two members
 * (`field48` a word, `field44` a byte), calls a second import veneer on
 * `this->field28` again (`fn3c4`, args: obj, 1), then returns the constant
 * 56 (0x38).
 *
 * Decompiled C (Ghidra, `FUN_00105be0`) reads exactly as modeled here —
 * see `.Data/Source_Extracted/decomp/cro_c/DllGts/00105be0_FUN_00105be0.c`.
 *
 * Reverse-engineering note: the only non-obvious part was instruction
 * SCHEDULING, not semantics. armcc's register allocator does `mov r4,r0`
 * (preserve `this`) immediately after the prologue, then evaluates the
 * two memory-backed call arguments in LEFT-TO-RIGHT source order
 * (`field28` first into r0, `field550` second into r3 via the just-saved
 * r4), and only then fills the two constant arguments (`38`, `1`) in
 * reverse (r2 then r1) right before the call. Writing the call as one
 * inline expression `fn214(this->field28, 1, 38, this->field550)` — or as
 * two locals assigned `field550` THEN `field28` — both made the compiler
 * evaluate `field550` before `field28` instead (3 of 15 words off:
 * swapped `mov r4,r0` / `ldr r0,[r0,#40]` / `ldr r3,[r4,#1360]` order).
 * Only assigning the locals in the SAME left-to-right order as the real
 * struct offsets (`field28` first, `field550` second) reproduced the
 * exact scheduling.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x5be0 0x3c \
 *       xy-decomp/armcc-matching/matched/DllGts/0x5be0.c
 */

typedef struct Obj {
    unsigned char pad1[0x28];
    void *field28;
    unsigned char pad2[0x44 - 0x28 - 4];
    unsigned char field44;
    unsigned char pad3[3];
    int field48;
    unsigned char pad4[0x550 - 0x48 - 4];
    int field550;
} Obj;

extern void *fn214(void *a, int b, int c, int d);
extern void *fn3c4(void *a, int b);

int F(Obj *this)
{
    void *f28 = this->field28;
    int f550 = this->field550;
    fn214(f28, 1, 38, f550);
    this->field48 = 0;
    this->field44 = 0;
    fn3c4(this->field28, 1);
    return 56;
}
