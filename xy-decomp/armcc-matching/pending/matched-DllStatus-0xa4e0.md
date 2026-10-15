Module: DllStatus
Offset: 0xa4e0
Size: 0xc
Match: 3/3
One-line: Byte-field setter taking its object through r2 (not r0): sets a flag byte at +0xc to 1, unconditionally returns 1.
Note: Sits right after a leaf tail-call-forwarder-shaped function at
0xa4cc (mov r3,#0; mov r0,r1; mov r2,r3; mov r1,r3; b <veneer>, not
attempted) and right before a real push{r4,r5,r6,lr} function at
0xa4ec. Unusual arg convention: object pointer arrives in the 3rd
register slot (r2), not r0/this -- reproduced in C with an (int, int,
Obj*) signature.
