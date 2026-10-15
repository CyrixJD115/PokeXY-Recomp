Module: DllGts
Offset: 0x5be0
Size: 0x3c
Match: 15/15
One-line: call+return helper — forwards two fields to an import veneer, zeroes two members, calls a second import veneer, returns constant 56.
Note: instruction-scheduling trap — armcc evaluates the two memory-backed call args in left-to-right SOURCE order (field28 then field550) while deferring the two constant args to the end in reverse order; writing the call inline, or assigning locals in the wrong order, produces a 3/15-word mismatch (swapped mov-r4/ldr-r0/ldr-r3 order) even though the semantics are identical.
