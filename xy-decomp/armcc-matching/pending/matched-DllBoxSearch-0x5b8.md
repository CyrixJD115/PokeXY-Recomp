Module: DllBoxSearch
Offset: 0x5b8
Size: 0x1c
Match: 7/7
One-line: Conditional accessor -- veneer call gates between a member load at +32 and a re-materialized constant 2.
Note: Same idiom as DllLangSelect@0x5b0 (matching pattern #6: the false branch re-materializes the literal 2 rather than reusing the veneer's own boolean result). This is a 43rd module, mined as the first real function right after the import-veneer table (0x180-0x5b7).
