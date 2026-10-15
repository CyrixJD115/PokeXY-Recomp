Module: DllBattleMatch
Offset: 0xde4
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder, entire body `b <import veneer>`.
Note: Same idiom as `DllBattleMatch@0xd90`/`DllBattleVideoSave@0x454`/
`DllDialogCommon@0x6e8` -- add to the cross-module idiom list the README
already tracks for this shape. Sits right after a destructor-shaped
function at `0xdc0` (same vptr-literal-load-vs-prologue-push residue as
the attempted `0xd98`, not re-attempted here) and right before another
bare `bx lr` stub (`0xde8`, also matched this run).
