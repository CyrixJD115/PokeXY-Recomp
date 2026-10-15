Module: DllSparring
Offset: 0x10178
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 8`, ignores `this`.
Note: First match in a new module (seventy-??th, exact count depends on
merge order with sibling agents). Sits in a run of three near-identical
2-word leaf constant-getters right after the module's second real
function (a dispatcher ending at 0x10174): `return 8` (claimed, 0x10178),
`return 16` (0x10180, unclaimed), `return 4` (0x10188, unclaimed). The
module's first real function (0x920, after a 244-entry import-veneer
table running 0x180-0x91f) is a large constructor, not attempted.
