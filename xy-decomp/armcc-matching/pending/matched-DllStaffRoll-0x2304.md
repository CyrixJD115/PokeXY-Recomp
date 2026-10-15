Module: DllStaffRoll
Offset: 0x2304
Size: 0x10
Match: 4/4
One-line: Thin double-forwarder: `return veneer(local(this));` — calls a local function then tail-calls another with its result as the argument.
Note: `push {r4,lr}`/`pop {r4,lr}` is AAPCS stack-alignment padding (r4 never referenced), same shape as DllDialogCommon@0x6c4's padding note. New compact variant of the forwarder family (previously only seen as single-call `b <veneer>` thin forwarders, e.g. DllBattleVideoSave@0x454/DllBattleMatch@0xd90/DllBattleVideoPlayer@0xb14/DllIntro@0x868) — this one chains a `bl` through a `b`.
