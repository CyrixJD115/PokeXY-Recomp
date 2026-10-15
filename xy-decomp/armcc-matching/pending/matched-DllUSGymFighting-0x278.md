Module: DllUSGymFighting
Offset: 0x278
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: A new (66th-ish) module. Same tail-of-import-veneer-table leaf-stub
cluster shape as every other Dll module mined so far: import-veneer table
runs 0x180-0x26f, then two bare `bx lr` stubs (0x270/0x274), this stub,
two more identical `mov r0,#0; bx lr` stubs at 0x280/0x288 (unclaimed),
two more bare `bx lr` stubs (0x290/0x294), then the module's first real
function (a 6-register-push dispatcher) at 0x298.
