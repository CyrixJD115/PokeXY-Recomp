Module: DllTownmap
Offset: 0x488
Size: 0x10
Match: 4/4
One-line: Calls a local module function (0x1f94) with `this` unchanged, discards result, returns 1.
Note: `push {r4,lr}` is AAPCS stack-alignment padding (r4 never touched), same idiom as DllDialogCommon@0x6c4's r4-push note. Right after the trivial stub cluster, before the dispatcher at 0x498.
