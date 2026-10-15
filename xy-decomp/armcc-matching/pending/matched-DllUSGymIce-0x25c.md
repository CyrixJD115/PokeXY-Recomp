Module: DllUSGymIce
Offset: 0x25c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: New module (sixty-third). Sits in the usual tail-of-import-veneer-table
leaf-stub cluster: bare `bx lr` at 0x258, this stub, an identical twin
`return 0` stub at 0x264 (unclaimed), four bare `bx lr` stubs at
0x268/0x26c/0x270/0x274, then a two-level-indirection sign-extended
byte-field getter at 0x278 (`ldr r0,[r0,#20]; ldr r0,[r0,#16]; sxtb r0,r0;
bx lr`), then the module's first real function at 0x288.
