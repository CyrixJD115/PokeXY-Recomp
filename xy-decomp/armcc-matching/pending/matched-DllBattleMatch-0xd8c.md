Module: DllBattleMatch
Offset: 0xd8c
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub, single `bx lr` (weak evidence, see note).
Note: Right before the already-matched `0xd90` thin forwarder, tail of the
import-veneer table. Weak-evidence match (any void no-op function compiles
to the same word) -- same caveat as `0xd94`/`DllDialogCommon`'s
`0x2f0`-`0x2fc` stubs.
