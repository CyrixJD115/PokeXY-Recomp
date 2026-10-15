Module: DllTestSuzuki
Offset: 0x180
Size: 0x2c
Match: 11/11
One-line: init function storing -1 into a 64-bit global and 0 into another (includes 2 literal-pool words)
Note: `volatile long long` on the first global is what yields the retail `strd` (new idiom; non-volatile long long compiles to mvn/mov/stm).
