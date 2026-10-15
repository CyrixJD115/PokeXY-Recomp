Module: DllField
Offset: 0x55384
Size: 0x38
Match: 14/14
One-line: Two-call helper -- CallA(this, this->f10, 0, 4, 8) then CallB(this->f0, 33); always returns 1.
Note: Same shape as DllField@0x54e08 (just-matched sibling, same module), different literal constants (0,4,8 and 33 vs 0,2,16 and 31). Real size is 0x38, not the hinted 0x40 -- same correction as 0x54e08 (hint span again included a trailing veneer tail-call stub and a bare bx lr).
