Module: DllSparringMenu
Offset: 0x5b14
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Sits right after the SetIsCallDraw setter at 0x5b0c (already matched), between
two near-identical conditional field-set leaves at 0x5b1c and 0x5b58 (untried; clamp
1/16/17 -> 2/3/4 and store into this-0x84+0x88 or +0x8c).
