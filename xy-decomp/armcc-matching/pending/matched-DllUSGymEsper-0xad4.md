Module: DllUSGymEsper
Offset: 0xad4
Size: 0x18
Match: 6/6
One-line: Array-indexed virtual dispatch -- looks up an owned object in an inline pointer array at this+0x24, tail-calls virtual slot 7 on it (new this, 3rd arg passed through).
Note: Identical idiom to DllUSBigFourSteel@0x2f4 (same member offset, same vtable slot 7) -- a thirty-second independent module confirming this array/vtable-indexing specimen recurs. Prologue-less, sits right after a prior function's pop{r4,r5,r6,r7,r8,pc} epilogue at 0xad0; a separate lone bx lr stub follows at 0xaec (not part of this function).
