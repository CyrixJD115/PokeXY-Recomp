Module: DllWazaOmoidashi
Offset: 0x3514
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: Right after a dispatcher's own literal-pool word (0x3510); four bare `bx lr` single-word stubs follow (0x351c/0x3520/0x3524/0x3528, unclaimed) before the next real function at 0x352c.
