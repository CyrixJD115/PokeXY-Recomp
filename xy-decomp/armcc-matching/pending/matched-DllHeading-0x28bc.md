Module: DllHeading
Offset: 0x28bc
Size: 0xc
Match: 3/3
One-line: Trivial field setter — zeroes a byte flag at `this+0xb4`, void return (`mov r1,#0; strb r1,[r0,#0xb4]; bx lr`).
Note: Sits in the same lone-`bx lr`-stub cluster shape documented for the other modules — three single-instruction no-op stubs at 0x28b0/0x28b4/0x28b8 immediately precede it, then the next function's full push-heavy prologue starts at 0x28c8. First function mined from this module; confirms the trivial-stub/simple-setter cluster near a dialog-proc base class's vtable tail recurs in a sixth independent Dll module. File: `xy-decomp/armcc-matching/matched/DllHeading/0x28bc.c`.
