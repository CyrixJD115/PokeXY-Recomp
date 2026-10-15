Module: DllTrainerCase
Offset: 0x3cd0
Size: 0x8
Match: 2/2
One-line: Trivial sub-object accessor, `add r0,r0,#28; bx lr` — returns `this+28` as a pointer.
Note: Same idiom as 0x3cc0 (`+16`) and 0x3cc8 (`+4`); second of the three `add r0,r0,#N; bx lr` siblings flagged in the module's README. One sibling (`0x3cd8` +8) and an untried thin tail-call forwarder (`0x3cb8`) plus a bare `bx lr` stub (`0x3cbc`) remain unclaimed for a future agent.
