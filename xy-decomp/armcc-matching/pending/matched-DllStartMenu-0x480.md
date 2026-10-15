Module: DllStartMenu
Offset: 0x480
Size: 0x40
Match: 16/16
One-line: Init helper — calls an import veneer on this->f24 with arg 0, sets this->f34=1 and this->f4->f2=1, then tail-calls a second veneer with constant args (0x60065, 0, -1, 0), returning its result.
Note: Tail-call idiom (pop {r4,lr}; mov r1,r3; b <veneer>) reuses r2/r3 values materialized early in the function (right after the first bl) for the final call's constant args — armcc filled idle scheduling slots mid-function. Natural source order reproduced it without manual reordering.
