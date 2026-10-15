Module: DllWazaOmoidashi
Offset: 0x5f0
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence.
Note: Sits right after the import-veneer table (0x180-0x5ec), immediately before the module's first real function (a constructor-shaped dispatcher at 0x5f4).
