Module: DllUSGymFairy
Offset: 0x224
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: A nineteenth module. Part of a run of three identical return-0 stubs
(0x224/0x22c/0x234) sitting right after a lone `bx lr` stub (0x220) and
before two more lone `bx lr` stubs (0x23c), then the module's first real
function at 0x240 -- the same recurring trivial-stub-cluster shape already
documented in every other mined module (import-veneer table runs
0x180-0x21f, 16 two-word entries). Only 0x224 claimed; 0x22c/0x234 are the
same shape (untried, likely equally easy) and the four bare `bx lr` stubs
at 0x220/0x23c are the weak-evidence class.
