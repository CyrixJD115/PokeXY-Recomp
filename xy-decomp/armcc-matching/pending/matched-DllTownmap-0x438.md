Module: DllTownmap
Offset: 0x438
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, return 1, ignores `this`.
Note: Right after the import-veneer table (0x180-0x437). Part of a run of three near-identical `return 1` stubs (0x438/0x440/0x448, only 0x438 claimed) and several bare `bx lr` weak-evidence stubs (0x450/0x454/0x460/0x464/0x468, unclaimed) before the already-matched conditional accessor at 0x46c.
