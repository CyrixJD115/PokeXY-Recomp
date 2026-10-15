Module: DllZukan
Offset: 0x1be54
Size: 0x8
Match: 2/2
One-line: Trivial field getter, returns the long at this+0x28.
Note: One of a run of three consecutive 2-word prologue-less getters
(0x1be54 +0x28, 0x1be5c +0xc, 0x1be64 +0x8) right after a `pop {r4,pc}`
epilogue at 0x1be50; followed by an array-indexed getter
(`add r0,r0,r1,lsl#2; ldr r0,[r0,#56]; bx lr`) at 0x1be6c, and a
`mov r0,#1`-style stub cluster at 0x1be78. The two sibling getters
(0x1be5c, 0x1be64) are unclaimed easy follow-ups for a future agent.

Also attempted and abandoned: 0x1194 (vtable-install + two import calls,
last one a tail call) — armcc always hoists the VT literal load before
the `push {r4,lr}` prologue and keeps `this` in r0 throughout (using r1
for VT, skipping a redundant `mov r0,r4` before the first call), while
the retail binary moves `this` into r4 immediately after the prologue,
loads VT into r0 afterward, and re-materializes r0=r4 before each call
(11 target words vs 10 candidate words — target has an extra `mov r0,r4`
the compiler always optimizes away). Looks like the same
vtable-pool-load-scheduling residue class already documented for the
dtor idiom in DllDialogCommon/DllNumberInput, just manifesting in a
ctor-shaped function instead of a dtor. Not re-attempted beyond two
variants (plain pointer cast and typed struct); a future agent may want
to try forcing register pressure (e.g. an extra dummy local) before
sinking more time into it.
