Module: DllDendouPC
Offset: 0x650
Size: 0x1c
Match: 7/7
One-line: Conditional accessor -- calls an import veneer with `this` unchanged, returns the member field at +36 if the call result is nonzero, else the literal 2.
Note: A new (forty-sixth) module. Same idiom already proven at DllLangSelect@0x5b0 (member offset 20), DllDendouDemo@0x798 (offset 32), DllBoxSearch@0x5b8 (offset 32) -- this specimen uses offset 36. Module's first real function, right after the import-veneer table (0x180-0x61f) and a trivial-stub cluster (0x620-0x64f: three `return 1` stubs, five bare `bx lr` weak-evidence stubs, none claimed).
