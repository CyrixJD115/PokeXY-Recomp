Module: DllZukan
Offset: 0x1dcc8 | Size: 0x8 | Match: 2/2 | One-line: returns this+4
Offset: 0x1dcd0 | Size: 0x8 | Match: 2/2 | One-line: returns this+0x1c
Offset: 0x1dce8 | Size: 0x8 | Match: 2/2 | One-line: returns this+0xc
Note: 0x1dcd8 (+8) and 0x1dce0 (+4) are identical shapes, not yet written up.
(0x1bad8 / 0x1baec were also verified by me but were already on dev from a sibling.)

### Unmatched: DllZukan @ 0x1bdc0 (size 0x58)
Shape: if (this[0xe0]) { a = vA(x) != 0; b = vB(x) != 0 }  return (vC(this)|a|b) != 0.
Best 9/22 (C int a=b=0, ternary). Init `mov r4,#0; mov r5,r4` matches; stuck on bool normalization:
target emits `cmp r0,#0; movne rN,#1; moveq rN,#0` (first call -> r4, second -> r5); armcc gives
`adds rN,r0,#0; movne` and swaps r4/r5. Tried int/ternary/temp/C++ bool (.cpp: 7/22).
