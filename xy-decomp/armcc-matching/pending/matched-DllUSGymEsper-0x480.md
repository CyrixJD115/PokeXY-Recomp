Module: DllUSGymEsper
Offset: 0x480
Size: 0x18
Match: 6/6
One-line: Boolean setter through one pointer indirection -- normalizes an int arg to 0/1 and stores it through this+20's pointee byte.
Note: Sibling of the boolean getter at 0x328 (same this+20 member, opposite direction). Source phrasing mattered: `v ? 1 : 0` / `v != 0` assigned directly makes armcc reuse the argument register in place (5 words, missing the target's explicit `mov r2,#0`); writing it as a separate `int flag = 0; if (v) flag = 1;` local, then storing that local, reproduces the target's unconditional-zero-then-conditional-one sequence through a distinct register exactly.
