Module: DllBox
Offset: 0x1f46c
Size: 0x10
Match: 4/4
One-line: Call-through-then-tail-call wrapper — `bl` one import veneer (0x9e0), pop, then tail-`b` into a second import veneer (0x258).
Note: Found via find_candidates.py's hint (Ghidra C: `FUN_001009e0(); FUN_00100258(); return;`), confirmed as two fixed no-arg calls where the second compiles as a tail branch since it's the function's last statement. `r4` push/pop around the `bl` is AAPCS stack-alignment padding, not a real saved value (same pattern already documented elsewhere in this project).
