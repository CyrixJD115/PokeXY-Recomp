/* DllUSPassage function @ CRO file offset 0x1610, size 0x4 — MATCHED (masked, 1/1). Thin
 * tail-call forwarder — entire body is a single `b <import veneer>` instruction (target
 * masked). Same idiom as this module's own 0x1078 (and DllBattleVideoSave@0x454/
 * DllBattleMatch@0xd90/DllBattleVideoPlayer@0xb14/DllIntro@0x868). This module alone has
 * four near-identical copies of this forwarder (0x1078, 0x1610, 0x1c80, 0x20c4) — all land
 * right after a small destructor-stage helper's own literal-pool word and right before a
 * run of bare `bx lr` no-op stubs.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSPassage 0x1610 0x4 xy-decomp/armcc-matching/matched/DllUSPassage/0x1610.c
 */

extern void veneer_call(void *this);

void tail_forward(void *this)
{
    veneer_call(this);
}
