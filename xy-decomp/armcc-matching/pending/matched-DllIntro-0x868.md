Module: DllIntro
Offset: 0x868
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder -- entire body is a single `b <import veneer>` instruction, same idiom as DllBattleVideoSave@0x454/DllBattleMatch@0xd90/DllBattleVideoPlayer@0xb14.
Note: First match in DllIntro (new module). Sits between a call-through wrapper at 0x858 (push {r4,lr}; bl <local 0x66cc>; pop {r4,lr}; b <import veneer 0>, untried, looks tractable -- same "call internal helper then tail-call import" shape) and a bare `bx lr` no-op stub at 0x86c (untried, weak evidence). The big function at 0x870 (push {r4-r8,lr}, VFP math, multi-branch) looks like a real dispatcher, not attempted.
