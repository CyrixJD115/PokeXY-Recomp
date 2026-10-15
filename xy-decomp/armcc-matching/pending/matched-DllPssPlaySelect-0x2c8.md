Module: DllPssPlaySelect
Offset: 0x2c8
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1` (void *this, ignored).
Note: A 23rd module. Same recurring tail-of-import-veneer-table leaf-stub
cluster shape as every other module mined so far: 24 `ldr pc,[pc,#-4]`
import veneers run 0x180-0x2c4, then two identical `return 1` stubs at
0x2c8/0x2d0 (only 0x2c8 claimed), three bare `bx lr` single-word stubs at
0x2d8/0x2dc/0x2e0 (untried, weak-evidence class), then the module's first
real function at 0x2e4 (a notification-dispatcher shape, same push
{r4,lr}/mov r4,r0 prologue as DllNumberInput@0x1890 — untried, plausible
next win for a future agent).
