Module: DllUSPassage
Offset: 0x334
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: A **forty-ninth-plus** new module. Sits in a run of three identical
return-0 stubs right after the import-veneer table (0x180-0x32f): a lone
`bx lr` at 0x330, this stub, another at 0x33c/0x340, another at
0x344/0x348, a final lone `bx lr` at 0x34c, then the module's first real
function (a large stack-frame-heavy function with calls back into its own
veneer table) at 0x350.
