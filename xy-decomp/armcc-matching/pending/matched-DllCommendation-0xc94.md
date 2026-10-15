Module: DllCommendation
Offset: 0xc94
Size: 0x38
Match: 14/14
One-line: Destructor: install vtable, free member @+0x14 if non-null, zero it, tail-call a second import with `this` in r0 (no `pop {r4,pc}` return).
Note: Eighth module for the technique. Same idiom as DllFatalErrorProc@0x284 and the "good" vtable-literal-load-before-member-load ordering from the toolchain's residue-class rule #4 (as opposed to the known-bad member-load-first ordering, which the module's neighboring function at 0xccc exhibits and was skipped on sight rather than attempted).
