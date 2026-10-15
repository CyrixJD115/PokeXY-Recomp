Module: DllUSGymFighting
Offset: 0x3f4
Size: 0x18
Match: 6/6
One-line: Conditional accessor — if this->f16==3, tail-call a veneer on this->f4 and return its result; else return `this`.
Note: Same family as DllLangSelect@0x5b0/DllDendouDemo@0x798/DllBoxSearch@0x5b8/
DllDendouPC@0x650/DllEggDemo@0x568/DllPCMenu@0x6b8, but the taken branch is a
tail call (`b`, not `bl`) and the fallback value is `this` itself rather than
a re-materialized literal. Size gotcha: real size is 0x18, not the naive 0x14
— the function's own trailing `bx lr` (the bne target) is part of its body;
the next function's unrelated `bx lr` stub starts right after, at 0x40c.
