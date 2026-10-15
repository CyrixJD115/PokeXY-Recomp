/* DllIntro @0x868, size 0x4 -- 1/1 masked match.
 *
 * Prologue-less tail-call forwarder: the entire function body is a single
 * `b <import veneer>` instruction (no push/pop, no other work). Sits
 * right after a small call-through wrapper at 0x858-0x867 (push {r4,lr};
 * bl <local 0x66cc>; pop {r4,lr}; b <import veneer 0>) and right before
 * another bare `bx lr` no-op stub at 0x86c -- the same tail-of-import-
 * veneer-table leaf-stub cluster shape already documented in this
 * module's siblings (DllBattleVideoSave@0x454, DllBattleMatch@0xd90,
 * DllBattleVideoPlayer@0xb14). A void->void tail call with no other
 * statements needs no stack frame, so armcc 5.04u5 at -O2 -Otime folds
 * the call straight into a bare `b` instead of `bl` + `pop {..,pc}`.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllIntro 0x868 0x4 \
 *       xy-decomp/armcc-matching/matched/DllIntro/0x868.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import0(void);

void fn868(void)
{
    import0();
}
