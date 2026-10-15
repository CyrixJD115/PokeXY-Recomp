/* DllSparringMenu function @ CRO file offset 0x5b58, size 0x34 — MATCHED
 * (masked, 13/13). Conditional field setter, this-pointer-adjusted
 * (an ARM C++ this-adjusting thunk shape: the real receiver is
 * `(char*)this - 0x84`). Clamps an input value to a small enum: v==1 -> 2,
 * v==16 -> 3, v==17 -> 4, stored into the adjusted object's field at
 * +0x8c; any other v leaves that field untouched. Void return.
 *
 * Sibling at 0x5b1c has the same cmp/moveq/beq cascade plus an extra
 * unconditional zero-store to a second field at +0x8c-and-+0x88 (offsets
 * swapped vs this one) that got stuck at 9/15 — armcc folds that extra
 * store's address differently depending on surrounding code, so it
 * wasn't re-attempted here; this simpler sibling (one field, no extra
 * store) matched on the first try with the obvious direct C translation.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSparringMenu 0x5b58 0x34 \
 *     xy-decomp/armcc-matching/matched/DllSparringMenu/0x5b58.c
 */
typedef struct S {
    unsigned char pad[0x8c];
    int f8c;
} S;

void fn(unsigned char *p, int v) {
    S *s = (S*)(p - 0x84);
    if (v == 1) s->f8c = 2;
    else if (v == 16) s->f8c = 3;
    else if (v == 17) s->f8c = 4;
}
