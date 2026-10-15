Module: DllEggDemo
Offset: 0x568
Size: 0x1c
Match: 7/7
One-line: Conditional accessor — veneer call gates between a member load at +24 and a re-materialized constant 3.
Note: 46th module. Same idiom already proven at DllLangSelect@0x5b0 (+20/else 2), DllDendouDemo@0x798 (+32/else 2), DllBoxSearch@0x5b8 (+32/else 2). Module's first real function, right after the import-veneer table (0x180-0x567, ~125 entries). Used XY_ROMFS=.Data/Source_Extracted/cro (the repo's pre-extracted CRO tree) since .Data/Raw_Extracted/romfs wasn't populated in this worktree.
