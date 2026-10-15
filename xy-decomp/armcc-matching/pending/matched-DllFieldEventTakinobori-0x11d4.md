Module: DllFieldEventTakinobori
Offset: 0x11d4
Size: 0x8
Match: 2/2
One-line: Trivial int field setter — stores the 2nd argument into the object at offset +0x18, void return (`str r1,[r0,#24]; bx lr`).
Note: A fifty-fifth new module. Sits right after a two-word literal pool (0x11cc/0x11d0) belonging to the preceding dispatcher function (which ends with `pop {r4,r5,r6,pc}` at 0x11c8), and immediately before a twin trivial `return 1` stub at 0x11dc (`mov r0,#1; bx lr`, size 0x8, untried) — both good candidates for a follow-up agent. The module's import-veneer table runs 0x180-0x367, after which the first real function (a float/VFP-heavy dispatcher) starts at 0x368.
