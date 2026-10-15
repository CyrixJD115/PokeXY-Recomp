Module: DllWazaOmoidashi
Offset: 0x924
Size: 0x14
Match: 5/5
One-line: Field setter — stores arg2 halfword into a sub-object at this+0x3c offset +14, then sets a byte flag at this+0x49 to 1.
Note: Register-scheduling trap — matching requires binding the constant `1` to its own local variable declared right after the sub-pointer load (before either store), so armcc hoists both loads (ldr r3,[r0,#60]; mov r2,#1) ahead of both stores (strh; strb) the way the retail binary does. Inlining the literal or swapping store order makes armcc interleave a load with its own store instead.
