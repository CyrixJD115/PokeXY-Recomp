/* DllBattleMatch @0xd90, size 0x4 -- 1/1 masked match.
 *
 * Prologue-less tail-call forwarder: the entire function body is a single
 * `b <import veneer>` instruction, sandwiched between two bare `bx lr`
 * no-op stubs (0xd8c and 0xd94) at the tail of the module's import-veneer
 * table. Same "thin forwarder" idiom as `DllBattleVideoSave@0x454` and
 * `DllDialogCommon@0x6e8` (that one via `bx r1` instead of a fixed `b`):
 * a void(ptr)->void tail call with no other work needs no stack frame, so
 * armcc 5.04u5 at -O2 -Otime folds the call straight into a bare `b`
 * instead of `bl`+`pop {..,pc}`.
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *       xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleMatch 0xd90 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleMatch/0xd90.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import184(void *o);

void fn_d90(void *o)
{
    import184(o);
}
