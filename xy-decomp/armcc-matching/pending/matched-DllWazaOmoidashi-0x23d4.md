Module: DllWazaOmoidashi
Offset: 0x23d4
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Right after a dispatcher's tail `b`; a bare `bx lr` stub (0x23dc) and a twin `return 1` stub (0x23e0) sit right next door, unclaimed.
