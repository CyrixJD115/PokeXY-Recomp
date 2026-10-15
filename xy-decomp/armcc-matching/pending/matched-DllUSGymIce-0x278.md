Module: DllUSGymIce
Offset: 0x278
Size: 0x10
Match: 4/4
One-line: Two-level-indirection sign-extended byte getter (`this->inner->field` narrowed to signed char).
Note: Residue-class finding worth folding into the toolchain doc: a plain
non-volatile `int field` at the target offset makes armcc 5.04u5 -O2
optimize the load down to `ldrb` once it sees only the low byte survives
the narrowing cast to `signed char` — this does NOT match. The retail
binary does a full-word `ldr` then `sxtb`. Marking the field `volatile`
blocks the optimization and reproduces the exact instruction pair. A
local `int` temp between the load and the cast was tried first and did
NOT help (still optimized to `ldrb`) — only `volatile` worked.
