Module: DllStatus
Offset: 0xbc8
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: New module (seventy-first?), sits right after the import-veneer
table (0x180-0xb87) in a run of trivial 1-2 word leaf stubs
(0xbc8-0xbff): four more identical `return 1` stubs at
0xbd0/0xbd8/0xbe4/0xbf8 (unclaimed) and four bare `bx lr` weak-evidence
stubs at 0xbe0/0xbec/0xbf0/0xbf4 (unclaimed), then the module's first
real function (a large dispatcher, push {r3-fp,lr}) at 0xc00.
