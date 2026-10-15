Module: DllUSDynamo
Offset: 0x1b4
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, returns 0 (false), ignores `this`.
Note: Thirteenth independent module mined. Small module (.text only 0x2b0
bytes); the import-veneer table runs 0x180-0x1af, then real functions start
at 0x1b0 with the usual trivial-stub cluster: lone `bx lr` @0x1b0 (weak
evidence), three identical `mov r0,#0; bx lr` stubs @0x1b4/0x1bc/0x1c4
(this one claimed), then two more lone `bx lr` stubs @0x1cc/0x1d0 (weak
evidence, unclaimed) before a real constructor-ish function at 0x1d4.
