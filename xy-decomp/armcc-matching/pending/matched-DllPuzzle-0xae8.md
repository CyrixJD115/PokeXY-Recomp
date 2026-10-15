Module: DllPuzzle
Offset: 0xae8
Size: 0x1c
Match: 7/7
One-line: Conditional field setter, prologue-less leaf, no calls: chases this->obj1(+4)->obj2(+0x2c), reads a byte at +0x60, and if it equals 1 writes literal 2 into a byte field at this+8. Void return.
Note: First function matched in this (new) module, right after the import-veneer table (which ends at 0xae4). Naive size guess 0x18 misses the trailing `bx lr`; real size is 0x1c. No branch immediates to mask (no bl/b in this specimen).
