/* DllBattleMatch @0xe38, size 0x10 -- 4/4 masked match.
 *
 * Two-call helper with a tail-call on the second: `push {r4,lr}; bl
 * <import A>(o); pop {r4,lr}; b <import B>` -- the `r4` push/pop is
 * AAPCS stack-alignment padding around the first (non-tail) call, same
 * idiom noted at `DllDialogCommon@0x6c4`. The key discovery making this
 * match: the second import call takes **no argument** (not `o`) -- an
 * attempt passing `o` to both calls forced armcc to insert `mov r4, r0`
 * / `mov r0, r4` to save/restore the pointer across the first call,
 * which the retail binary does not do. Once the second call was made
 * argument-less, `r0` (== `o`) simply isn't live across the first call
 * at all, and no save/restore is needed -- the `pop {r4,lr}; b <B>` at
 * the end is then a plain tail call (restores the real return address
 * into `lr` before jumping, so `<import B>`'s own `bx lr` returns
 * straight to this function's original caller).
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *       xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleMatch 0xe38 0x10 \
 *       xy-decomp/armcc-matching/matched/DllBattleMatch/0xe38.c
 *   -> target 4 words, candidate 4 words, 4/4 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import_a(void *o);
extern void import_b(void);

void fn_e38(void *o)
{
    import_a(o);
    import_b();
}
