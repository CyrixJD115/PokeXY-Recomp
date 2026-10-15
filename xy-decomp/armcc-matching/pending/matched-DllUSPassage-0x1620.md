Module: DllUSPassage
Offset: 0x1620
Size: 0x8
Match: 2/2
One-line: Trivial field getter, returns the signed byte at `this+0x5c`.
Note: Sits right after a run of bare `bx lr` no-op stubs (0x1614-0x161c)
and right before a real constructor-shaped function at 0x1628.
