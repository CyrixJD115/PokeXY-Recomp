Module: DllTradelist
Offset: 0xc70
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence.
Note: Sits between the known-bad member-load-first dtor sibling at 0xc3c (skipped on sight) and the large 10-register-push dispatcher at 0xc74. Fifth and final match this run (hit the 5-function ceiling).
