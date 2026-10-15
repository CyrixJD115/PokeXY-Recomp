Module: DllBattleVideoSave
Offset: 0x454
Size: 0x4
Match: 1/1
One-line: Prologue-less single-instruction tail-call forwarder (`b <import veneer>`), a new module (thirty-second so far).
Note: Sits right after a dispatcher function (0x3f0-0x448) and its 2-word literal pool (0x44c/0x450); next function (0x458, `bx lr`, untried) is a twin no-op/forwarder-style stub. Same "thin forwarder" family as DllDialogCommon@0x6e8's `bx r1`, but via a direct masked `b` instead of an indirect branch -- void-tail-call C with no other statements reproduces it on the first try since armcc 5.04u5 needs no stack frame and folds the call into a plain `b`.
