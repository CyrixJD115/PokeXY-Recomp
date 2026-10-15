Module: DllTownmap
Offset: 0xb10
Size: 0x1c
Match: 7/7
One-line: virtual-call wrapper: if (this->[0x88]) call its vtable slot 4 passing through arg1 (tail call via bx).
Note: Arg1 pass-through is what forces the vtable load into r2; a 1-arg void() signature gives r1 and mismatches.
