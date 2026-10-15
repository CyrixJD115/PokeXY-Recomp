Module: DllPhotoViewer
Offset: 0x1918
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`.
Note: A twenty-third module. Sits in a run of seven near-identical 1-2 word
leaf stubs at 0x1918-0x193b (another identical `return 1` pair at 0x1920,
five bare `bx lr` single-word stubs at 0x1928/0x192c/0x1930/0x1934/0x1938 —
untried), right after a literal-pool word at 0x1914 and right before a real
`push {r4,lr}`-prologue function at 0x193c. Same recurring trivial-stub
cluster shape documented for every other Dll module mined so far.
