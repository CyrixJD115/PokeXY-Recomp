Module: DllSubScreenTutorial
Offset: 0x12a0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, return 1, ignores this.
Note: One of three identical "mov r0,#1; bx lr" stubs (0x12a0/0x12a8/0x12b0), followed by five bare bx-lr single-word stubs (0x12b8-0x12c8, weak-evidence class) before the dtor at 0x12cc.
