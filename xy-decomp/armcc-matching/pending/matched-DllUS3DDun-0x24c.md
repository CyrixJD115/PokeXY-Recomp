Module: DllUS3DDun
Offset: 0x24c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: Thirteenth module mined. Sits inside a run of 9 tiny stub functions
(0x248-0x273: lone `bx lr` / `mov r0,#0; bx lr` pairs) immediately after
the import veneer table (0x180-0x247) and right before the first real
function at 0x274 — same recurring trivial-stub cluster shape documented
for every other Dll module, just located at the head of real code
instead of the tail this time. Several more untried stubs remain in that
same run for a future agent (0x248, 0x254, 0x25c, 0x260, 0x264, 0x268,
0x26c, 0x270).
