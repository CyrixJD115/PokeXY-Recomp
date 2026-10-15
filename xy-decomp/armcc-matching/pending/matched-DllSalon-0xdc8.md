Module: DllSalon
Offset: 0xdc8
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this` — an eighth module confirming the trivial-stub-cluster pattern.
Note: Sits immediately after a destructor-stage helper (vptr install + conditional member free, same idiom as DllDialogCommon/DllNumberInput/DllFatalErrorProc) ending in `pop {r4,pc}` at 0xdc0 plus its own literal-pool word at 0xdc4; the stub occupies exactly 0xdc8-0xdcf (`mov r0,#0; bx lr`), and the next function (0xdd0, a -1/zero-init field setter) starts immediately after. `.text` segment for this module is at CRO file offset 0x180, size 0xd2c (parsed from the segment table at 0x200c).
