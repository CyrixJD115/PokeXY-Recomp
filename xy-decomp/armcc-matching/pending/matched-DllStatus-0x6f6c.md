Module: DllStatus
Offset: 0x6f6c
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder -- entire body is a single `b <import veneer>` instruction.
Note: Same idiom as DllBattleVideoSave@0x454/DllBattleMatch@0xd90/
DllBattleVideoPlayer@0xb14/DllIntro@0x868. Sandwiched between two bare
`bx lr` single-word stubs (0x6f68/0x6f70, weak-evidence class,
unclaimed) mid-module (not near the import-veneer table).
