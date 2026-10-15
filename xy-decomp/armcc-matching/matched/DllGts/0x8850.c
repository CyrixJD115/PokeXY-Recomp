/*
 * DllGts @ 0x8850, size 0x28, masked match 10/10.
 *
 * Call+return helper: double-indirects through `this->field20->field5c`
 * (a nested object pointer) and forwards it plus four `1` constants to
 * an import veneer with 5 args (4 in registers + 1 on the stack, AAPCS
 * overflow): `fn284(ptr, 1, 1, 1, 1)`, returns the constant 8.
 *
 * Decompiled C (Ghidra, `FUN_00108850`):
 *   FUN_00100284(*(undefined4 *)(*(int *)(param_1 + 0x20) + 0x5c),1,1,1,1);
 *   return 8;
 * — see `.Data/Source_Extracted/decomp/cro_c/DllGts/00108850_FUN_00108850.c`.
 *
 * Reverse-engineering note — the real blocker here was NOT the pointer
 * chase (a local `Inner *p = this->field20;` before the call reproduces
 * that part on the first try) but which REGISTER armcc's constant-CSE
 * picks as the "master" copy of the four identical `1`s. With a plain
 * `fn284(p->field5c, 1, 1, 1, 1)` call (even with distinct locals for
 * every `1`, or none at all), armcc consistently materializes the
 * shared immediate into **r1** (arg2's register) and copies it outward
 * from there (stack, r3, r2) — but the retail binary materializes it
 * into **r2** (arg3's register) instead and copies outward from THERE
 * (stack, r3, r1). Both are valid codegens of the same semantics; only
 * the register choice differs, which still mismatches 6/10 words. Fix:
 * wrap ONLY the arg3 occurrence of the constant in its own named local
 * (`int v3 = 1;`) while leaving the other three `1`s as literals —
 * `fn284(p->field5c, 1, v3, 1, 1)`. That nudge is enough to make armcc's
 * CSE pick r2 (the local's natural register) as the master, exactly
 * matching retail. (A `fn284(p->field5c, v2, 1, 1, 1)` naming arg2
 * instead would presumably flip it back to r1 — untested, since arg3
 * naming already gave the exact match.) The identical trick applies to
 * the sibling at `0x12afc` (same shape, field at offset 0x24 instead of
 * 0x20, returns 7 instead of 8) — already verified to
 * `*** MASKED MATCH ***` with the same source shape, just not committed
 * here to respect this run's 5-function batch ceiling; it's a ~1-minute
 * pickup for whoever goes next (same template, swap offset+return).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x8850 0x28 \
 *       xy-decomp/armcc-matching/matched/DllGts/0x8850.c
 */

typedef struct Inner {
    unsigned char pad[0x5c];
    void *field5c;
} Inner;

typedef struct Obj {
    unsigned char pad[0x20];
    Inner *field20;
} Obj;

extern void *fn284(void *a, int b, int c, int d, int e);

int F(Obj *this)
{
    Inner *p = this->field20;
    int v3 = 1;
    fn284(p->field5c, 1, v3, 1, 1);
    return 8;
}
