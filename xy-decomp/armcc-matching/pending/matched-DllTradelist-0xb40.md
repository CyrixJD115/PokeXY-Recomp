Module: DllTradelist
Offset: 0xb40
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in a new (71st) module. Sits in the tail-of-import-veneer-table leaf-stub cluster (0xb40-0xc03): two more identical `return 1` stubs at 0xb50/0xb68 (unclaimed) and six bare `bx lr` weak-evidence stubs interleaved, then a real constructor at 0xb70.
