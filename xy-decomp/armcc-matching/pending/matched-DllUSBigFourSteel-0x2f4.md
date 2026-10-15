Module: DllUSBigFourSteel
Offset: 0x2f4
Size: 0x18
Match: 6/6
One-line: Array-indexed virtual dispatch -- looks up an owned object pointer in an inline pointer array at this+0x24 (indexed by arg1), then tail-calls virtual slot 7 on it with the object as the new `this` and the caller's original third arg passed through.
Note: Twenty-eighth module mined. Sits right after the recurring leaf-stub cluster (import-veneer table 0x180-0x2cf, then bare `bx lr` / `return 0` stubs at 0x2d0-0x2f0); this is the module's first real function. A lone `bx lr` stub follows at 0x30c (untried, weak-evidence class). Matched on the first try -- no residue-class gotchas hit.
