/* DllBattleMatch @0xde4, size 0x4 -- 1/1 masked match.
 *
 * Thin tail-call forwarder: entire body is a single `b <import veneer>`
 * instruction, same idiom as the already-matched `0xd90` in this module
 * (and `DllBattleVideoSave@0x454`/`DllDialogCommon@0x6e8`): a
 * void(ptr)->void tail call with no other work needs no stack frame, so
 * armcc 5.04u5 at -O2 -Otime folds the call straight into a bare `b`
 * instead of `bl`+`pop {..,pc}`. Sits right after a destructor-shaped
 * function (`0xdc0`, attempted/stuck on the same vptr-literal-load-vs-
 * prologue-push residue as `0xd98`) and right before another bare `bx lr`
 * stub (`0xde8`).
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *       xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleMatch 0xde4 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleMatch/0xde4.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import184(void *o);

void fn_de4(void *o)
{
    import184(o);
}
