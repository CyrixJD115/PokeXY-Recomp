Module: DllBattle
Offset: 0x30a4
Size: 0x68
Match: 26/26 (relaxed: needs --relax-nops, one nop word)
One-line: destructor - vtable store, release owned object at +0xc via vcall slots 4/1, free +0x38, base sub-object dtor at +0x18, return this
Note: vtable pool-load before member load shape works. Sibling 0x3038 (0x6c, same + tail call b) stuck 21/26: retail hoists the vtable ldr above push, armcc does not.
