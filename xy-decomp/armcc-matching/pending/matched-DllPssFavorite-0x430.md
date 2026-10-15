Module: DllPssFavorite
Offset: 0x430
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`.
Note: A thirty-third module. Import-veneer table runs 0x180-0x42f (84 entries,
unusually large), then this stub and a twin at 0x438, then three bare `bx lr`
single-word stubs at 0x440/0x444/0x448 (weak-evidence class, untried), then
the module's first real function (a constructor-shaped dispatcher calling
the veneer at 0x180) at 0x44c.
