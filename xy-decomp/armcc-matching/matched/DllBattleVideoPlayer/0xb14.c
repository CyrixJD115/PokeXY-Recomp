/* DllBattleVideoPlayer @0xb14, size 0x4 -- 1/1 masked match.
 *
 * Prologue-less tail-call forwarder: the entire function body is a single
 * `b <import veneer>` instruction. Sits in a run of 8 near-identical
 * one-word leaf functions (0xb14-0xb33: alternating `b 0x188` tail-call
 * forwarders and bare `bx lr` no-op stubs) right after a veneer-calling
 * wrapper ending at 0xb0c (plus its own literal-pool word at 0xb10) and
 * right before the next full-prologue function at 0xb34. Same "thin
 * forwarder" idiom already proven at DllBattleVideoSave@0x454 and
 * DllDialogCommon@0x6e8 -- a void->void tail call with no other
 * statements needs no stack frame, so armcc 5.04u5 at -O2 -Otime emits a
 * direct `b` instead of `bl` + `bx lr`. The `b`'s 24-bit immediate is
 * masked by the verifier (link-time fact), so the exact veneer target is
 * unverified -- only the single-instruction tail-call shape is proven.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleVideoPlayer 0xb14 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleVideoPlayer/0xb14.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import188(void);

void fnb14(void)
{
    import188();
}
