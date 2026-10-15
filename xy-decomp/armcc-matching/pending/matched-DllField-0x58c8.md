Module: DllField
Offset: 0x58c8
Size: 0xc
Match: 3/3
One-line: Trivial hardcoded-true byte-field setter — unconditionally sets this+0x1c to 1, ignores any second argument, void return.
Note: First function mined from DllField (a new, 72nd module — large module, 9227 functions per the tracker). Sits right after the import-veneer table (0x180-0x5767) and a run of small veneer-calling wrappers/float helper functions (0x5740-0x58c4); the next function starts immediately after at 0x58d4. Plenty of unmined ground remains in this module.
