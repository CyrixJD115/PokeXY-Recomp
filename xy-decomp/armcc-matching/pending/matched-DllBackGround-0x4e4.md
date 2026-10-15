Module: DllBackGround
Offset: 0x4e4
Size: 0x24
Match: 9/9
One-line: Boolean predicate on a byte field at this+0x164 — returns 1 if the field equals 22, 11, or 63, else 0. No calls (prologue-less leaf), right after the module's import-veneer table (0x180-0x4d7) and a tiny VFP one-liner at 0x4d8.
Note: First match in a thirty-eighth module (not in the README's count yet). A clean three-way-equality leaf, not a stub: cmp/cmpne/beq handles the first two equalities (22, 11) sharing the branch target with the fallthrough cmp/movne/bne handling the third (63) — both paths converge on the same `mov r0,#1 / bx lr` tail. No branch-immediate masking was actually exercised since all branches are short intra-function forward jumps, not link-time external targets.
