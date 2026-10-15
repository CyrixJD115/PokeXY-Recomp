Module: DllPromotion
Offset: 0xc78
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in this module (new module for the table). Sits at the
tail of the import-veneer table (0x180-0xc27), right after a small
constructor at 0xc28 and a float-field-init leaf at 0xc54 (each with
their own literal pools ending at 0xc50/0xc74). Followed by six bare
`bx lr` single-word stubs (0xc80-0xc90, unclaimed weak-evidence class)
and then the module's first real function (a 4-register-push
dispatcher) at 0xc94.
