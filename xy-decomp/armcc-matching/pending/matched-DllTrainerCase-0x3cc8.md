Module: DllTrainerCase
Offset: 0x3cc8
Size: 0x8
Match: 2/2
One-line: Trivial sub-object accessor, `add r0,r0,#4; bx lr` — returns `this+4` as a pointer.
Note: Same idiom as the already-matched 0x3cc0 (`+16`); first of the three `add r0,r0,#N; bx lr` siblings flagged in the module's README (`0x3cc8` +4, `0x3cd0` +28, `0x3cd8` +8).
