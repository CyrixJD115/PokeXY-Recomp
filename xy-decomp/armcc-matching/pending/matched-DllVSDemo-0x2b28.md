Module: DllVSDemo
Offset: 0x2b28
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: Right after a destructor-stage helper (0x2b20) and its literal-pool word
(0x2b24), same cluster shape as 0x3e0/0x1a60/0x1b00 in this module.
Also attempted (not matched, logged separately): DllVSDemo@0x2984, a
prologue-less tail-virtual-call leaf (same idiom as DllDialogCommon@0x6e8) —
got stuck at 5/8, the vtable-temp register never reproduces (armcc always
picks r3, retail uses r1, regardless of struct shape or -O1/-O2/-O3/-Ospace).
