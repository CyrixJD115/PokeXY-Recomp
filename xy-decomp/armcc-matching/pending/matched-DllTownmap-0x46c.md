Module: DllTownmap
Offset: 0x46c
Size: 0x1c
Match: 7/7
One-line: Conditional accessor — calls import veneer (0x180) on `this`, returns member +0x10 if nonzero else literal 3.
Note: First match in this module (62nd+ module overall). Same idiom as DllLangSelect@0x5b0/DllDendouDemo@0x798/DllBoxSearch@0x5b8/DllDendouPC@0x650/DllEggDemo@0x568/DllPCMenu@0x6b8. Module layout: import-veneer table 0x180-0x437, then a run of trivial 1-2 word stubs (0x438-0x46b: three `return 1` stubs, several bare `bx lr`), then this function, then a run of larger dispatcher/destructor-shaped functions starting at 0x488.
