Module: DllKisekae
Offset: 0xb6e0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in this module (a new, previously-unmined module). Sits
right after a destructor-stage helper (vptr install, conditional member
free via masked `bl`, return `this`) ending at 0xb6db (own literal pool at
0xb6dc). An identical twin `return 1` stub follows at 0xb6e8 (unclaimed),
then a `mov r1,#1; strb r1,[r0,#0x40a]; bx lr` hardcoded-true byte-field
setter at 0xb6f0 (untried, 0xc bytes, doesn't use its own argument), then
the module's next real function (7-register-push dispatcher) at 0xb6fc.
Import-veneer table runs 0x180-0x90f (~232 entries); plenty more of the
usual recurring stub/dtor-idiom cluster shape likely elsewhere in this
~0xb80c-byte .text for a future agent.
