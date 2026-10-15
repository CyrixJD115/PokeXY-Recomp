Module: DllBattleMatch
Offset: 0xe38
Size: 0x10
Match: 4/4
One-line: Two-call helper: `import_a(o)` then a tail call to argument-less
`import_b()`.
Note: Non-trivial (stronger evidence than the module's stub matches).
Gotcha worth folding in: the second call must take NO argument -- passing
`o` to both calls makes armcc insert `mov r4,r0`/`mov r0,r4` save/restore
pairs around the first call that the retail binary doesn't have. Once the
second call's signature dropped to `void(void)`, `o` simply isn't live
across the first call and the `pop {r4,lr}; b <import_b>` tail-call falls
out naturally (restores the real `lr` before jumping, so `import_b`'s own
`bx lr` returns straight to this function's caller).
