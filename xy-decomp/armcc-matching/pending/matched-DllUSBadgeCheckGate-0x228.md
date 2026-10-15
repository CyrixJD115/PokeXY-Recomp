Module: DllUSBadgeCheckGate
Offset: 0x228
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, returns 0 (false), ignores `this`.
Note: A thirteenth module. Sits right after the import-veneer table
(0x180-0x218) and four bare single-word `bx lr` stubs (0x218/0x21c/0x220/
0x224), with two more identical return-0 stubs immediately following at
0x230 and 0x238 (untried, same shape). Same idiom already seen in
DllCommWait/DllSalon/DllConfig's tail-stub clusters.
