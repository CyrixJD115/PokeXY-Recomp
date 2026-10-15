Module: DllWazaOboe
Offset: 0x82c
Size: 0xc
Match: 3/3
One-line: Field setter + return 1 — sets a byte flag at +12 of the object passed in r2 (3rd argument), ignores `this` and the 2nd argument, returns 1.
Note: Right before the module's first real function at 0x838 (constructor-shaped, push {r4-r8,lr}; vpush {d8} — unattempted).
