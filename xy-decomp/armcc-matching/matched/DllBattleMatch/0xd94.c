/* DllBattleMatch @0xd94, size 0x4 -- 1/1 masked match.
 * Style: natural
 *
 * Trivial no-op void stub (single `bx lr`), immediately following the
 * `0xd90` thin forwarder in the same already-matched run at the tail of
 * the module's import-veneer table. Weak-evidence match: any void
 * function with no observable side effect compiles to this same one
 * word, so the match proves only that the real body is at most this
 * short -- same caveat as the four `0x2f0`-`0x2fc` stubs in
 * `DllDialogCommon`.
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *       xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleMatch 0xd94 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleMatch/0xd94.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
void fn_d94(void *o)
{
    (void)o;
}
