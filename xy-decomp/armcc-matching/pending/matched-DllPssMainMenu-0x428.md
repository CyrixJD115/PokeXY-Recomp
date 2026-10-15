Module: DllPssMainMenu
Offset: 0x428
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A seventy-fourth matched module. Sits right after the import-veneer
table (0x180-0x427): an identical twin `return 1` stub follows at 0x430
(unclaimed), then two bare `bx lr` single-word weak-evidence stubs at
0x438/0x43c, then the module's first real function (a dispatcher with
several veneer calls and a shared epilogue, same general shape as the
notification-dispatcher idiom seen elsewhere) starts at 0x440.
