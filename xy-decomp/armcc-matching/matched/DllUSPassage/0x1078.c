/* DllUSPassage function @ CRO file offset 0x1078, size 0x4 — MATCHED (masked, 1/1). Thin
 * tail-call forwarder: the entire function body is a single `b <import veneer>` instruction
 * (target masked — a link-time fact). Same idiom as DllBattleVideoSave@0x454/
 * DllBattleMatch@0xd90/DllBattleVideoPlayer@0xb14/DllIntro@0x868: a void->void tail call with
 * no other statements needs no stack frame, so armcc 5.04u5 at -O2 -Otime folds it straight
 * into a bare `b` instead of `bl`+`pop {..,pc}`. Sits right after a small destructor-stage
 * helper's own literal-pool word (0x1074) and right before a bare `bx lr` no-op stub
 * (0x107c).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPassage 0x1078 0x4 xy-decomp/armcc-matching/matched/DllUSPassage/0x1078.c
 */

extern void veneer_call(void *this);

void tail_forward(void *this)
{
    veneer_call(this);
}
