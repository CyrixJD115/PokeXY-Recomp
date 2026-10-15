/* DllBackGround function @ CRO file offset 0x4e4, size 0x24 — MATCHED (masked, 9/9).
 * First matched function from a thirty-eighth module. A prologue-less leaf
 * boolean predicate: reads a byte flag at this+0x164 and returns 1 if it
 * equals 22, 11, or 63, else 0. No calls, no branch-immediate masking needed
 * in practice (all branches are short internal forward branches within the
 * function, not link-time facts). The three-way equality compiles to a
 * cmp/cmpne/beq pair (first two equalities) falling through to a second
 * cmp/movne/bne (third equality) sharing the same `mov r0,#1 / bx lr` tail
 * for both the first-pair hit and the straight-through third-equality hit.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBackGround 0x4e4 0x24 xy-decomp/armcc-matching/matched/DllBackGround/0x4e4.c
 */

typedef struct X {
    unsigned char pad[0x164];
    unsigned char field;   /* +0x164 */
} X;

int func(X *o)
{
    unsigned char f = o->field;
    return (f == 22 || f == 11 || f == 63) ? 1 : 0;
}
