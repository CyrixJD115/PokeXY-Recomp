Module: DllUSR13
Offset: 0x24c
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`, ignores `this`.
Note: A twenty-third mined module. Same recurring stub cluster right after
the import-veneer table (0x180-0x247): a lone `bx lr` at 0x248, three
`mov r0,#0; bx lr` stubs at 0x24c/0x254/0x25c (only 0x24c claimed), two more
bare `bx lr` stubs at 0x264/0x268, then the module's first real function
(a constructor-shaped loop) at 0x26c.
