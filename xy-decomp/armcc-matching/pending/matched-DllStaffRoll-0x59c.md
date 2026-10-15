Module: DllStaffRoll
Offset: 0x59c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in a new module (seventy-first-ish by this swarm's running count — check README's current tally before inserting). Sits right after the import-veneer table (0x180-0x598); three more identical `return 1` stubs at 0x5a4/0x5ac/0x5b4 and five bare `bx lr` weak-evidence stubs at 0x5bc-0x5cc are unclaimed. The module's first real function (0x5d0) is a 5-case jump-table dispatcher — same known-hard jump-table/NOP-hint residue class already logged for DllDialogCommon@0xdd8/DllCommWait@0xce0 — not attempted.
