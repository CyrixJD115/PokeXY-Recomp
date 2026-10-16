/* DllBattleMatch @0xde8, size 0x4 -- 1/1 masked match.
 * Style: natural
 *
 * Trivial no-op void stub (single `bx lr`), right after the `0xde4`
 * thin forwarder. Weak-evidence match: any void function with no
 * observable side effect compiles to this same one word, so the match
 * proves only that the real body is at most this short -- same caveat
 * as `0xd8c`/`0xd94` in this module.
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *       xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleMatch 0xde8 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleMatch/0xde8.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
void fn_de8(void *o)
{
    (void)o;
}
