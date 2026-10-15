Module: DllUSKinomi
Offset: 0xf40
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder — entire body is a single `b <import veneer>` instruction.
Note: First match in a **seventy-first** module (new module not previously in the README table). Same thin-forwarder idiom as DllBattleVideoSave@0x454/DllBattleMatch@0xd90/DllBattleVideoPlayer@0xb14/DllIntro@0x868. Sits in a small stub cluster right after the module's first (large) real function: forwarder@0xf40, two bare `bx lr` stubs @0xf44/0xf48 (0xf44 also matched this batch), three identical `return 0` stubs @0xf4c/0xf54/0xf5c (all three matched this batch), one more bare `bx lr` @0xf64 (untried), then a big real function at 0xf68.
