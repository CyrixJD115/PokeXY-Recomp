Module: DllUSPassage
Offset: 0x1078
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder — entire body is a single `b <import veneer>` instruction.
Note: Same idiom as DllBattleVideoSave@0x454/DllBattleMatch@0xd90/
DllBattleVideoPlayer@0xb14/DllIntro@0x868. Sits right after a small
destructor-stage helper's own literal-pool word (0x1074) and right before
a bare `bx lr` no-op stub (0x107c).
