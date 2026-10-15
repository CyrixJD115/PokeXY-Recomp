Module: DllTrainerCase
Offset: 0x3cc0
Size: 0x8
Match: 2/2
One-line: Trivial sub-object accessor, returns `(char*)this + 16` (pointer arithmetic, no load).
Note: New stub-idiom variant (`add r0,r0,#N; bx lr` instead of `mov r0,#N`). Three untried siblings follow (0x3cc8 +4, 0x3cd0 +28, 0x3cd8 +8), plus an untried thin tail-call forwarder at 0x3cb8 and a bare bx-lr stub at 0x3cbc. Good leads for the next agent in this module.
